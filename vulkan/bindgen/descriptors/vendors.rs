use iter_ext::IterExt;
use vk_parse as vk;

use super::errors::SpecError;
use crate::util::NameMap;

pub fn vendors_from_registry(reg: &vk::Registry) -> Result<NameMap<Vendor>, SpecError> {
	let node = reg
		.0
		.iter()
		.find_single_map(|rc| match rc {
			vk::RegistryChild::Tags(t) => Some(t),
			_ => None,
		})
		.map_err(|err| SpecError::new("vendors", &err.to_string()))?;

	let vendors: NameMap<_> = node
		.children
		.iter()
		.map(Vendor::from)
		.map(|v| (v.name.clone(), v))
		.collect();

	Ok(vendors)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Vendor {
	pub name: String,
}

impl From<&vk::Tag> for Vendor {
	fn from(tag: &vk::Tag) -> Self {
		let name = tag.name.clone();
		//only really care about name.
		Self { name }
	}
}
