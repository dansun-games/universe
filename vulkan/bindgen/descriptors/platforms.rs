use iter_ext::IterExt;
use vk_parse as vk;

use super::errors::SpecError;
use crate::util::NameMap;


pub fn platforms_from_registry(reg: &vk::Registry) -> Result<NameMap<Platform>, SpecError> {
	let node = reg
		.0
		.iter()
		.find_single_map(|rc| match rc {
			vk::RegistryChild::Platforms(p) => Some(p),
			_ => None,
		})
		.map_err(|err| SpecError::new("platforms", &err.to_string()))?;

	let platforms: NameMap<_> = node
		.children
		.iter()
		.map(Platform::from)
		.map(|v| (v.name.clone(), v))
		.collect();

	Ok(platforms)
}


#[derive(Debug, PartialEq, Eq)]
pub struct Platform {
	pub name: String,
}

impl From<&vk::Platform> for Platform {
	fn from(plat: &vk::Platform) -> Self {
		let name = plat.name.clone();

		//See https://github.com/KhronosGroup/Vulkan-Docs/issues/2297 
		//for why we dont store more information than this

		Self { name }
	}
}
