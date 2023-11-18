use core::panic;

use vk_parse as vk;

use super::alias::Alias;

pub fn get_enums(reg: &vk::Registry) -> Vec<EnumDescriptor> {
	let mut enums: Vec<_> = reg.0
		.iter()
		.filter_map(|item| match item {
			vk::RegistryChild::Enums(v) => Some(v),
			_ => None,
		})
		.filter(|e| e.kind.is_some())
		.map(|e| EnumDescriptor::from(e))
		.collect();

	patch_enums(&mut enums);
	enums
}

pub fn get_enum_aliases(enums: &Vec<EnumDescriptor>, types: &Vec<vk::Type>) -> Vec<Alias> {
	let enums_types = types
		.iter()
		.filter(|c| c.category.as_ref().is_some_and(|c| c == "enum"));

	let enum_defs = enums_types.clone().filter(|c| c.alias.is_none());
	for def in enum_defs {
		let search_name = def.name.clone().expect("Enum missing name");
		let matched = enums.iter().find(|e| e.name == search_name);
		if matched.is_none() {
			panic!("Could not find enum {:?} referenced in type list", search_name)
		}
	}

	types
		.iter()
		.filter(|c| c.category.as_ref().is_some_and(|c| c == "enum") && c.alias.is_some())
		.map(Alias::from)
		.collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumDescriptor {
	pub name:       String,
	pub is_bitmask: bool,
	pub bit_width: u32,
	pub values:     Vec<EnumValue>,
	pub aliases:    Vec<EnumAlias>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumValue {
	pub name:  String,
	pub value: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumAlias {
	pub name:  String,
	pub alias: String,
}

impl From<&vk::Enums> for EnumDescriptor {
	fn from(def: &vk::Enums) -> Self {
		// No longer used.
		assert_eq!(def.start, None);
		assert_eq!(def.end, None);
		assert_eq!(def.vendor, None);
		
		let kind = def.kind.as_ref().expect("Missing Kind for enum");
		let is_bitmask = match kind.as_str() {
			"bitmask" => true,
			"enum" => false,
			_ => panic!("Invalid kind for enum"),
		};
		
		let bit_width = def.bitwidth.unwrap_or(32);
		let name = def.name.as_ref().expect("Missing name").clone();

		let enum_defs = def.children.iter().filter_map(|ec| match ec {
			vk::EnumsChild::Enum(e) => Some(e),
			_ => None,
		});

		let values: Vec<_> = enum_defs
			.clone()
			.filter_map(|e| match &e.spec {
				vk::EnumSpec::Value { value, extends } => {
					assert_eq!(extends.as_ref(), None);
					Some(EnumValue {
						name:  e.name.clone(),
						value: parse_enum_val(value),
					})
				},
				vk::EnumSpec::Bitpos { bitpos, extends } => {
					assert_eq!(extends.as_ref(), None);
					Some(EnumValue {
						name:  e.name.clone(),
						value: 2_i64.pow(*bitpos as u32),
					})
				},
				vk::EnumSpec::Offset { .. } => panic!("Enum offset not supported."),
				_ => None,
			})
			.collect();

		let aliases: Vec<_> = enum_defs
			.clone()
			.filter_map(|e| match &e.spec {
				vk::EnumSpec::Alias { alias, extends } => {
					assert_eq!(extends.as_ref(), None);
					Some(EnumAlias {
						name:  e.name.clone(),
						alias: alias.clone(),
					})
				},
				_ => None,
			})
			.collect();

		EnumDescriptor {
			name,
			is_bitmask,
			bit_width,
			values,
			aliases,
		}
	}
}

fn parse_enum_val(value: &str) -> i64 {
	if value.starts_with("0x") {
		i64::from_str_radix(&value[2..], 16)
	} else {
		i64::from_str_radix(&value, 10)
	}
	.unwrap()
}

fn patch_enums(enums: &mut Vec<EnumDescriptor>) {
	let patch_names = vec!["VkQueryPoolCreateFlagBits"];
	
	let conflicting: Vec<_> = enums
		.iter()
		.filter(|e| patch_names.contains(&e.name.as_str()))
		.map(|e| e.name.as_str())
		.collect();

	if conflicting.len() > 0 {
		panic!("Patch enums are already defined: {:?}", conflicting);
	}

	enums.push(EnumDescriptor { 
		name: String::from("VkQueryPoolCreateFlagBits"), 
		is_bitmask: true, 
		bit_width: 32,
		values: vec![],
		aliases: vec![],
	});

	enums.push(EnumDescriptor { 
		name: String::from("VkDeviceCreateFlagBits"), 
		is_bitmask: true, 
		bit_width: 32,
		values: vec![],
		aliases: vec![],
	});
}