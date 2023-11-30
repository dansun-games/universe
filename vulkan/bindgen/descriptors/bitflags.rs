use vk_parse as vk;

use crate::util::{NameMap, iter_spec_types};

use super::alias::Alias;

const BITFLAGS_KIND: &str = "bitmask";

//bitmask values are a little different than most types
//they are utilizing the enum tags for bitmasks.
//they are stored in a different place in the spec, and required by the enum type tag.
pub fn get_bitflags(reg: &vk::Registry) -> (NameMap<BitflagDescriptor>, NameMap<Alias>) {
	let bitflag_bits: NameMap<BitflagDescriptor> = reg
		.0
		.iter()
		.filter_map(|item| match item {
			vk::RegistryChild::Enums(v) => Some(v),
			_ => None,
		})
		.filter(|e| e.kind.as_ref().is_some_and(|k| k == BITFLAGS_KIND))
		.map(BitflagDescriptor::from)
		.map(|b| (b.name.clone(), b))
		.collect();

	let bitflags: NameMap<BitflagDescriptor> = iter_spec_types(&reg)
		.filter(|c| c.category.as_ref().is_some_and(|c| c == BITFLAGS_KIND) && c.alias.is_none())
		.map(|t| {
			let name = t.name.clone().expect("bitflag missing name");
			BitflagDescriptor {
				name,
				bit_width: 32,
				values: vec![],
				aliases: vec![]
			}
		})
		.map(|desc| (desc.name.clone(), desc))
		.collect();

	(NameMap::new(), NameMap::new())
}


#[derive(Debug, PartialEq, Eq)]
pub struct BitflagDescriptor {
	pub name: String,
	pub bit_width: usize,
	pub values: Vec<BitflagValue>,
	pub aliases: Vec<Alias>,
}

//the name is confusing, but vk::Enums is a single enum definition
impl From<&vk::Enums> for BitflagDescriptor {
    fn from(def: &vk::Enums) -> Self {
		assert_eq!(def.start, None);
		assert_eq!(def.end, None);
		assert_eq!(def.vendor, None);

		let kind = def.kind.as_ref().expect("missing kind");
		assert_eq!(kind, BITFLAGS_KIND);

		let name = def.name.clone().expect("Missing name");
		let bit_width = def.bitwidth.unwrap_or(32) as usize;

		let bitmask_defs = def.children.iter().filter_map(|ec| match ec {
			vk::EnumsChild::Enum(e) => Some(e),
			_ => None,
		});

		let values: Vec<_> = bitmask_defs
			.clone()
			.map(|e| match &e.spec {
				vk::EnumSpec::Bitpos { bitpos, extends } => {
					assert_eq!(extends.as_ref(), None);
					BitflagValue {
						name: e.name.clone(),
						value: 2_i64.pow(*bitpos as u32),
					}
				},
				_ => panic!("Invalid EnumSpec value"),
			})
			.collect();

		let aliases: Vec<_> = bitmask_defs
			.clone()
			.map(|e| match &e.spec {
				vk::EnumSpec::Alias { alias, extends } => {
					assert_eq!(extends.as_ref(), None);
					Alias {
						name: e.name.clone(),
						alias_for: alias.clone(),
					}
				},
				_ => panic!("Invalid EnumSpec value"),
			})
			.collect();

        BitflagDescriptor { 
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
	pub value: i64,
}

/*


fn patch_enums(enums: &mut NameMap<EnumDescriptor>) {
	let new_enums = vec![
		EnumDescriptor {
			name: String::from("VkQueryPoolCreateFlagBits"),
			is_bitmask: true,
			bit_width: 32,
			values: vec![],
			aliases: vec![],
		},
		EnumDescriptor {
			name: String::from("VkDeviceCreateFlagBits"),
			is_bitmask: true,
			bit_width: 32,
			values: vec![],
			aliases: vec![],
		},
	];
	let new_names: Vec<_> = new_enums.iter().map(|v| &v.name).collect();

	let conflicting: Vec<_> = enums.keys().filter(|v| new_names.contains(v)).collect();

	if conflicting.len() > 0 {
		panic!("Patch enums are already defined: {:?}", conflicting);
	}

	enums.extend(&mut new_enums.into_iter().map(|v| (v.name.clone(), v)));
}

*/
