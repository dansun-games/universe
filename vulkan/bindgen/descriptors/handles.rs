use vk_parse as vk;

use super::alias::Alias;

pub fn get_handles(types: &Vec<vk::Type>) -> (Vec<HandleDesc>, Vec<Alias>) {
	let handle_types = types.iter().filter(|t| t.category.as_ref().unwrap() == "handle");
	let handles: Vec<_> = handle_types.clone().map(HandleDesc::from).collect();
	let aliases: Vec<_> = handle_types.clone().map(Alias::from).collect();

	(handles, aliases)
}

pub struct HandleDesc {
	name: String,
	parent: Option<HandleParent>,
	object_type: String,
}

impl From<&vk::Type> for HandleDesc {
	fn from(def: &vk::Type) -> Self {
		assert_eq!(def.category.as_ref().unwrap(), "handle");
		let name = def.name.clone().expect("Handle missing name");
		let parent = def
			.parent
			.as_ref()
			.map(String::as_ref)
			.map(HandleParent::from);
		let object_type = def.objtypeenum.clone().expect("Missing object type");

		HandleDesc {
			name,
			parent,
			object_type,
		}
	}
}

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
