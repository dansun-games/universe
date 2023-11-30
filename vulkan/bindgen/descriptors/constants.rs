use std::collections::HashMap;

use vk_parse as vk;

use super::alias::Alias;

#[derive(Debug, PartialEq, Eq)]
pub struct ConstDescriptor {
	pub name: String,
	pub c_type: String,
	pub value: String,
}

impl From<&vk::Enum> for ConstDescriptor {
	fn from(def: &vk::Enum) -> Self {
		assert_eq!(def.api, None);
		assert_eq!(def.protect, None);
		assert_eq!(def.deprecated, None);

		let name = def.name.clone();
		let c_type = def.type_suffix.clone().expect("Could not get const type");
		let value = match def.spec.clone() {
			vk::EnumSpec::Value { value, extends } => {
				assert_eq!(extends, None);
				value
			},
			_ => panic!("Spec is not enum value"),
		};

		ConstDescriptor {
			name,
			c_type,
			value,
		}
	}
}

pub fn get_constants(
	reg: &vk::Registry,
) -> (HashMap<String, ConstDescriptor>, HashMap<String, Alias>) {
	let def_iter = reg
		.0
		.iter()
		.filter_map(|item| match item {
			vk::RegistryChild::Enums(v) => Some(v),
			_ => None,
		})
		.find(|t| t.name.as_ref().is_some_and(|v| v == "API Constants"))
		.expect("Could not find constants block")
		.children
		.iter()
		.filter_map(|e| match e {
			vk::EnumsChild::Enum(v) => Some(v),
			_ => None,
		});

	let enums = def_iter
		.clone()
		.filter(|e| e.type_suffix.is_some())
		.map(ConstDescriptor::from)
		.map(|v| (v.name.clone(), v))
		.collect::<HashMap<String, ConstDescriptor>>();

	let aliases = def_iter
		.clone()
		.filter(|e| e.type_suffix.is_none())
		.map(Alias::from)
		.map(|v| (v.name.clone(), v))
		.collect::<HashMap<String, Alias>>();

	(enums, aliases)
}
