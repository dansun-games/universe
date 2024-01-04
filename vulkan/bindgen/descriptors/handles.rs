
use vk_parse as vk;

use super::alias::Alias;
use crate::util::{iter_spec_types, NameMap};

pub fn get_handles(
	reg: &vk::Registry,
) -> (NameMap<HandleDescriptor>, NameMap<Alias>) {
	let handle_types = iter_spec_types(reg).filter(|t| t.category.as_ref().is_some_and(|c| c == "handle"));
	let handles: NameMap<_> = handle_types
		.clone()
		.filter(|v| v.alias.is_none())
		.map(HandleDescriptor::from)
		.map(|v| (v.name.clone(), v))
		.collect();
	let aliases: NameMap<_> = handle_types
		.clone()
		.filter(|v| v.alias.is_some())
		.map(Alias::from)
		.map(|v| (v.name.clone(), v))
		.collect();

	(handles, aliases)
}

#[derive(Debug, PartialEq, Eq)]
pub struct HandleDescriptor {
	pub name: String,
	pub parent: Option<HandleParent>,
	pub rust_type: String,
	pub object_type: String,
}

impl From<&vk::Type> for HandleDescriptor {
	fn from(def: &vk::Type) -> Self {
		assert_eq!(def.category.as_ref().unwrap(), "handle");

		let spec = match &def.spec {
			vk::TypeSpec::Code(v) => v,
			_ => panic!("Unexpected handle spec type"),
		};

		let (name, dispatchable) = if let Some(s) = spec.code.strip_prefix("VK_DEFINE_HANDLE") {
			(unwrap_paren(s), true)
		} else if let Some(s) = spec.code.strip_prefix("VK_DEFINE_NON_DISPATCHABLE_HANDLE") {
			(unwrap_paren(s), false)
		} else {
			panic!("Unexpected handle define macro");
		};
		let name = name.to_owned();

		//"dispatchable handles" are actual pointers to memory, hence they should be the actual size of pointers on the device.
		//"non dispatchable handles" on the other hand, are more like id's to opaque resources. Vulkan always uses 64-bit ids for these.
		let rust_type = if dispatchable {
			String::from("usize")
		} else {
			String::from("u64")
		};

		let object_type = def.objtypeenum.clone().expect("Missing object type enum");
		let parent = def
			.parent
			.as_ref()
			.map(String::as_ref)
			.map(HandleParent::from);

		HandleDescriptor {
			name,
			parent,
			rust_type,
			object_type,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandleParent {
	Instance,
	Device,
	PhysicalDevice,
	CommandPool,
	DescriptorPool,
	Display,
	VideoSession,
}

impl From<&str> for HandleParent {
	fn from(value: &str) -> Self {
		use HandleParent::*;
		match value {
			"VkInstance" => Instance,
			"VkDevice" => Device,
			"VkPhysicalDevice" => PhysicalDevice,
			"VkCommandPool" => CommandPool,
			"VkDescriptorPool" => DescriptorPool,
			"VkDisplayKHR" => Display,
			"VkVideoSessionKHR" => VideoSession,
			_ => panic!("Unknown handle parent value"),
		}
	}
}


fn unwrap_paren(s: &str) -> &str {
	s.strip_prefix("(")
		.expect("Missing opening paren")
		.strip_suffix(")")
		.expect("Missing closing paren")
}
