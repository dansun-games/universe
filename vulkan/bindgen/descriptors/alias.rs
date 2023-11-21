use vk_parse as vk;

#[derive(Debug, PartialEq, Eq)]
pub struct Alias {
	pub name: String,
	pub alias_for: String,
}

impl From<&vk::Type> for Alias {
	fn from(def: &vk::Type) -> Self {
		assert_eq!(def.api, None);
		assert_eq!(def.bitvalues, None);
		assert_eq!(def.deprecated, None);
		assert_eq!(def.objtypeenum, None);
		assert_eq!(def.parent, None);
		assert_eq!(def.requires, None);
		assert_eq!(def.spec, vk::TypeSpec::None);
		assert_eq!(def.structextends, None);
		assert_eq!(def.allowduplicate, None);
		assert_eq!(def.returnedonly, None);

		let name = def.name.clone().unwrap();
		let alias_for = def.alias.clone().unwrap();

		Alias { name, alias_for }
	}
}

impl From<&vk::Enum> for Alias {
	fn from(def: &vk::Enum) -> Self {
		assert_eq!(def.api, None);
		assert_eq!(def.protect, None);
		assert_eq!(def.deprecated, None);
		assert_eq!(def.type_suffix, None);

		let name = def.name.clone();
		let alias_for = match def.spec.clone() {
			vk::EnumSpec::Alias { alias, extends } => {
				assert_eq!(extends, None);
				alias
			},
			_ => panic!("Spec is not enum value"),
		};

		Alias { name, alias_for }
	}
}
