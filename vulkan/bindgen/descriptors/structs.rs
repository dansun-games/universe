use std::collections::HashMap;

use vk_parse as vk;

use super::var::VarDescriptor;
use crate::descriptors::alias::Alias;

pub fn get_structs(
	types: &Vec<vk::Type>,
) -> (HashMap<String, StructDescriptor>, HashMap<String, Alias>) {
	let all_types = types
		.iter()
		.filter(|t| t.category.as_ref().is_some_and(|cat| cat == "struct"));

	let structs = all_types
        .clone()
        .filter(|t| t.alias.is_none()) //dont care about aliasing
		.map(|t| t.into())
        .map(|v: StructDescriptor| (v.name.clone(), v))
		.collect::<HashMap<String, StructDescriptor>>();

	let aliases = all_types
		.clone()
		.filter(|t| t.alias.is_some())
		.map(Alias::from)
		.map(|v| (v.name.clone(), v))
		.collect::<HashMap<String, Alias>>();

	(structs, aliases)
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructMember {
	pub var_spec: VarDescriptor,
	pub variant_feat: Option<String>,
	pub deprecated: bool,
	pub sync_ctrl: bool,
}

impl From<&vk::TypeMemberDefinition> for StructMember {
	fn from(def: &vk::TypeMemberDefinition) -> Self {
		assert_eq!(def.selection, None);
		assert_eq!(def.validextensionstructs, None);

		// Used in returnonly structs to specificy how the properties are to be used?
		// &def.limittype;

		// Just says the param isnt error checked by the runtime.
		// &def.noautovalidity;

		let variant_feat = def.api.clone();
		let deprecated = def
			.deprecated
			.as_ref()
			.inspect(|v| assert_eq!(v, &"ignored"))
			.is_some();
		let sync_ctrl = def
			.externsync
			.as_ref()
			.inspect(|v| assert_eq!(v, &"true"))
			.is_some();

		let var_spec = def.into();

		StructMember {
			var_spec,
			variant_feat,
			deprecated,
			sync_ctrl,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructDescriptor {
	pub name: String,
	pub parents: Vec<String>,
	pub members: Vec<StructMember>,
}

impl From<&vk::Type> for StructDescriptor {
	fn from(def: &vk::Type) -> Self {
		assert_eq!(def.category.as_ref().unwrap(), "struct");
		assert_eq!(def.alias, None);
		assert_eq!(def.api, None);
		assert_eq!(def.bitvalues, None);
		assert_eq!(def.deprecated, None);
		assert_eq!(def.objtypeenum, None);
		assert_eq!(def.parent, None);
		assert_eq!(def.requires, None);

		let name = def.name.as_ref().unwrap().to_owned();

		let members = match &def.spec {
			vk_parse::TypeSpec::Members(v) => v,
			_ => panic!("struct specification has wrong kind."),
		};
		let members: Vec<_> = members
			.iter()
			.filter_map(|m| match m {
				vk_parse::TypeMember::Definition(d) => Some(d),
				_ => None,
			})
			.map(StructMember::from)
			.collect();

		// used to specify whether an extension or feature can be specified multiple times within the XML file? Pretty useless because the definition only happens once.
		// &def.allowduplicate;

		// used to specify whether a struct is not used in any parameters. Seems not useful.
		// &def.returnedonly;

		let parents: Vec<_> = def
			.structextends
			.as_ref()
			.map(|v| v.split(",").map(|v| v.to_owned()).collect())
			.unwrap_or_default();

		StructDescriptor {
			name,
			parents,
			members,
		}
	}
}
