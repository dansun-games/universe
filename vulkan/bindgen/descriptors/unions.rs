use vk_parse as vk;

use crate::descriptors::var::VarDescriptor;

pub fn get_unions(types: &Vec<vk::Type>) -> Vec<UnionDescriptor> {
	types
		.iter()
		.filter(|item| item.category.as_ref().unwrap() == "union")
		.map(UnionDescriptor::from)
		.collect()
}


pub struct UnionDescriptor {
	pub name: String,
	pub members: Vec<VarDescriptor>,
}

impl From<&vk::Type> for UnionDescriptor {
	fn from(def: &vk::Type) -> Self {
		assert_eq!(def.category.as_ref().unwrap(), "union");
		assert_eq!(def.api, None);
		assert_eq!(def.alias, None);
		assert_eq!(def.bitvalues, None);
		assert_eq!(def.deprecated, None);
		assert_eq!(def.parent, None);
		assert_eq!(def.requires, None);
		assert_eq!(def.structextends, None);
		assert_eq!(def.allowduplicate, None);
		// assert_eq!(def.returnedonly, None);

		let name = def.name.clone().unwrap();
		let members = match &def.spec {
			vk::TypeSpec::Members(v) => v,
			_ => panic!("Members not found in enum."),
		};

		let members: Vec<_> = members
			.iter()
			.filter_map(|m| match m {
				vk::TypeMember::Definition(v) => Some(v),
				_ => None,
			})
			.map(VarDescriptor::from)
			.collect();

		UnionDescriptor { name, members }
	}
}
