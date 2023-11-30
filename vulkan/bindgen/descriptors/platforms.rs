use std::collections::HashMap;

use vk_parse as vk;

pub fn get_platforms(reg: &vk::Registry) -> HashMap<String, String> {
	let mut search = reg.0.iter().filter_map(|item| match item {
		vk::RegistryChild::Platforms(v) => Some(v),
		_ => None,
	});

	let platforms = search.next().expect("Could not find platforms.");
	assert_eq!(search.next(), None);

	platforms
		.children
		.iter()
		.map(|v| (v.name.clone(), v.name.clone()))
		.collect::<HashMap<String, String>>()
}
