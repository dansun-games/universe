use vk_parse as vk;

use super::alias::Alias;
use crate::util::{iter_spec_types, NameMap};

const BITFLAGS_CAT: &str = "bitmask";

//bitmask values are a little different than most types
//they are utilizing the enum tags for bitmasks.
//they are stored in a different place in the spec, and required by the enum type tag.
pub fn get_bitflags(reg: &vk::Registry) -> (NameMap<BitflagDescriptor>, NameMap<Alias>) {
	let mut bitflags: NameMap<BitflagDescriptor> = iter_spec_types(&reg)
		.filter(|c| c.category.as_ref().is_some_and(|c| c == BITFLAGS_CAT) && c.alias.is_none())
		.map(BitflagDescriptor::from)
		.map(|bf| (bf.name.clone(), bf))
		.collect();

	let bitflag_aliases: NameMap<Alias> = iter_spec_types(&reg)
		.filter(|c| c.category.as_ref().is_some_and(|c| c == BITFLAGS_CAT) && c.alias.is_some())
		.map(Alias::from)
		.map(|a| (a.name.clone(), a))
		.collect();

	let mut bitflag_bits: NameMap<BitflagBits> = reg
		.0
		.iter()
		.filter_map(|item| match item {
			vk::RegistryChild::Enums(v) => Some(v),
			_ => None,
		})
		.filter(|e| e.kind.as_ref().is_some_and(|k| k == BITFLAGS_CAT))
		.map(BitflagBits::from)
		.map(|b| (b.name.clone(), b))
		.collect();
	
	//merge our BitflagDescriptor's with the correct bits
	for bitflag in bitflags.values_mut() {
		if let Some(bt) = bitflag.bits_name.as_ref() {
			let bits = bitflag_bits.remove(bt).expect("Could not find bitflag bits");
			bitflag.values = bits.values;
			bitflag.aliases = bits.aliases;
			
			//can remove if once https://github.com/KhronosGroup/Vulkan-Headers/issues/462 is resolved
			// if bitflag.name != "VkPhysicalDeviceSchedulingControlsFlagsARM" {
			// 	assert_eq!(bitflag.bit_width, bits.bit_width);
			// }
		}
	}

	//empty bits dont have their flag associated.
	for (name, bits) in bitflag_bits.drain() {
		assert_eq!(bits.values.len(), 0);
		assert_eq!(bits.aliases.len(), 0);
		let bitflag_name = name.replace("FlagBit", "Flag");
		let bitflag = bitflags.get_mut(&bitflag_name).expect("Could not find bitflag from orphan bits");
		bitflag.bits_name = Some(name);
	}
	
	let missing_bits: Vec<_> = bitflags.values().filter(|v| v.bits_name.is_none()).collect();	
	println!("{:#?}", missing_bits);

	assert_eq!(bitflag_bits.len(), 0);

	(bitflags, bitflag_aliases)
}

#[derive(Debug, PartialEq, Eq)]
pub struct BitflagDescriptor {
	pub name: String,
	pub bit_width: usize,
	pub bits_name: Option<String>,
	pub api_feat: Option<String>,
	pub values: Vec<BitflagValue>,
	pub aliases: Vec<Alias>,
}

impl From<&vk::Type> for BitflagDescriptor {
	fn from(def: &vk::Type) -> Self {
		//TODO: assert
		let api_feat = def.api.clone();
		let name_bits = def.requires.clone().or(def.bitvalues.clone());

		//parse the code
		let type_code = match &def.spec {
			vk::TypeSpec::Code(c) => c,
			_ => panic!("could not extract code from spec"),
		};
		let type_code = &type_code.code;
		let type_code = type_code.strip_prefix("typedef").expect("typedef missing");
		let type_code = type_code.strip_suffix(";").expect("missing simicolon");

		let mut iter = type_code.trim().split_whitespace();
		let c_type = iter.next().expect("missing c_type");
		let name = iter.next().expect("missing name").to_owned();
		assert_eq!(iter.next(), None);

		let bit_width = match c_type {
			"VkFlags" => 32,
			"VkFlags64" => 64,
			_ => panic!("invalid c_type for bitflag"),
		};

		if name == "VkPipelineColorBlendStateCreateFlags" {
			println!("{:?}", name_bits);
		}

		BitflagDescriptor {
			name,
			bits_name: name_bits,
			bit_width,
			api_feat,
			values: vec![],
			aliases: vec![],
		}
	}
}


#[derive(Debug, PartialEq, Eq)]
struct BitflagBits {
	name: String,
	bit_width: usize,
	values: Vec<BitflagValue>,
	aliases: Vec<Alias>,
}

//the name is confusing, but vk::Enums is a single enum definition
impl From<&vk::Enums> for BitflagBits {
	fn from(def: &vk::Enums) -> Self {
		assert_eq!(def.start, None);
		assert_eq!(def.end, None);
		assert_eq!(def.vendor, None);

		let kind = def.kind.as_ref().expect("Missing kind");
		assert_eq!(kind, BITFLAGS_CAT);

		let name = def.name.clone().expect("Missing name");
		let bit_width = def.bitwidth.unwrap_or(32) as usize;

		let bitmask_defs = def.children.iter().filter_map(|ec| match ec {
			vk::EnumsChild::Enum(e) => Some(e),
			_ => None,
		});

		let values: Vec<_> = bitmask_defs
			.clone()
			.filter_map(|e| match &e.spec {
				vk::EnumSpec::Bitpos { bitpos, extends } => {
					assert_eq!(extends.as_ref(), None);
					Some(BitflagValue {
						name: e.name.clone(),
						bitpos: *bitpos as u8,
					})
				},
				_ => None,
			})
			.collect();

		let aliases: Vec<_> = bitmask_defs
			.clone()
			.filter_map(|e| match &e.spec {
				vk::EnumSpec::Alias { alias, extends } => {
					assert_eq!(extends.as_ref(), None);
					Some(Alias {
						name: e.name.clone(),
						alias_for: alias.clone(),
					})
				},
				_ => None,
			})
			.collect();

		BitflagBits {
			name,
			bit_width,
			values,
			aliases,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct BitflagValue {
	pub name: String,
	pub bitpos: u8,
}

impl BitflagValue {
}
