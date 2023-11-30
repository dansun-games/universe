
use vk_parse as vk;

use crate::util::NameMap;

pub fn get_platforms(reg: &vk::Registry) -> NameMap<String> {
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
		.collect::<NameMap<String>>()
}
