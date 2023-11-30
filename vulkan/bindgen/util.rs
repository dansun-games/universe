use std::collections::HashMap;

use vk_parse as vk;

pub type NameMap<T> = HashMap<String, T>;

pub fn iter_spec_types<'a>(reg: &'a vk::Registry) -> impl 'a + Iterator<Item = &vk::Type> + Clone {
	let mut filtered = reg.0.iter().filter_map(|item| match item {
		vk::RegistryChild::Types(v) => Some(v),
		_ => None,
	});

	let types = filtered.next().expect("Could not find types");
	assert_eq!(filtered.next(), None);

	types
		.children
		.iter()
		.filter_map(|t| match t {
			vk_parse::TypesChild::Type(v) => Some(v),
			_ => None,
		})
		// Ignore items that dont have a category.
		// These will have to have special handling. Not sure how yet.
		.filter(|t| t.category.is_some())
		// Ignore includes.
		.filter(|t| t.category.as_ref().unwrap() != "include")
		// Ignore #defines for now.. just going to manually create those.
		// There arent that many and its not immediately clear how to convert it.
		.filter(|t| t.category.as_ref().unwrap() != "define")
}
