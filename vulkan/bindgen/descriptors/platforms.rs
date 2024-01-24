use vk_parse as vk;

use super::errors::SpecError;

pub struct Platform {
	pub name: String,
	pub vendor: String,
}

impl TryFrom<&vk::Platform> for Platform {
	type Error = SpecError;

	fn try_from(plat: &vk::Platform) -> Result<Self, Self::Error> {
		let name = plat.name.clone();
		let flag_prefix = format!("VK_USE_PLATFORM_{}", name.to_uppercase());

		let mut vendor = plat
			.protect
			.strip_prefix(&flag_prefix)
			.ok_or_else(|| SpecError::new(&format!("platform/{name}"), &"Invalid use flag prefix"))?
			.to_owned();

		//workaround. see: https://github.com/KhronosGroup/Vulkan-Docs/issues/2297
		if name == "sci" {
			vendor = "NV".to_owned();
		} else if vendor.is_empty() {
			vendor = name.to_uppercase();
		}

		Ok(Self { name, vendor })
	}
}
