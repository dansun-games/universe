
use std::collections::HashSet;

use vk_parse as vk;

use super::alias::Alias;
use crate::util::{iter_spec_types, NameMap};

//enum values are a little different than most types
//they are stored in a different place in the spec, and required by the enum type tag.
pub fn get_enums(reg: &vk::Registry) -> (NameMap<EnumDescriptor>, NameMap<Alias>) {
	let enum_requires: HashSet<_> = iter_spec_types(&reg)
		.filter(|t| t.category.as_ref().is_some_and(|c| c == "enum") && t.alias.is_none())
		.map(|t| t.name.clone().unwrap()) //only care about name
		.collect();

	let aliases: NameMap<_> = iter_spec_types(&reg)
		.filter(|c| c.category.as_ref().is_some_and(|c| c == "enum") && c.alias.is_some())
		.map(Alias::from)
		.map(|v| (v.name.clone(), v))
		.collect();

	let enums: NameMap<_> = reg
		.0
		.iter()
		.filter_map(|item| match item {
			vk::RegistryChild::Enums(v) => Some(v),
			_ => None,
		})
		.filter(|e| e.kind.as_ref().is_some_and(|k| k == "enum"))
		.map(EnumDescriptor::from)
		.filter(|desc| enum_requires.contains(&desc.name)) //filter out any enums that arent required
		.map(|desc| (desc.name.clone(), desc))
		.collect();

	(enums, aliases)
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumDescriptor {
	pub name: String,
	pub values: Vec<EnumValue>,
	pub aliases: Vec<Alias>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumValue {
	pub name: String,
	pub value: i64,
}

//the name is confusing, but vk::Enums is a single enum definition
impl From<&vk::Enums> for EnumDescriptor {
	fn from(def: &vk::Enums) -> Self {
		assert_eq!(def.start, None);
		assert_eq!(def.end, None);
		assert_eq!(def.vendor, None);
		assert_eq!(def.bitwidth, None);
		let kind = def.kind.as_ref().expect("Missing kind");
		assert_eq!(kind, "enum");

		let name = def.name.as_ref().expect("Missing name").to_owned();

		let enum_defs = def.children.iter().filter_map(|ec| match ec {
			vk::EnumsChild::Enum(e) => Some(e),
			_ => None,
		});

		let values: Vec<_> = enum_defs
			.clone()
			.filter_map(|e| match &e.spec {
				vk::EnumSpec::Value { value, extends } => {
					assert_eq!(extends.as_ref(), None); //never seen this used
					Some(EnumValue {
						name: e.name.clone(),
						value: parse_enum_val(value),
					})
				},
				_ => None,
			})
			.collect();

		let aliases: Vec<_> = enum_defs
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

		EnumDescriptor {
			name,
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
