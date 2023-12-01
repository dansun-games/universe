
use vk_parse as vk;

use crate::util::NameMap;

pub fn get_extensions(reg: &vk::Registry) -> NameMap<ExtensionDescriptor> {
	let mut filtered = reg.0.iter().filter_map(|item| match item {
		vk::RegistryChild::Extensions(exts) => Some(exts),
		_ => None,
	});

	let extensions = filtered.next().expect("Could not find extensions");
	assert_eq!(filtered.next(), None);

	extensions
		.children
		.iter()
		.filter(|ext| match ext.supported.as_ref() {
			Some(sup) => sup != "disabled",
			None => false,
		})
		.map(ExtensionDescriptor::from)
		.map(|v| (v.v_name.clone(), v))
		.collect()
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExtensionDeprecation {
	VersionPromotion { major: u32, minor: u32 },
	Rename(String),
}

impl From<&str> for ExtensionDeprecation {
	fn from(value: &str) -> Self {
		if value.starts_with("VK_VERSION") {
			let mut it = value.split('_').skip(2);
			let major: u32 = it
				.next()
				.expect("not enough parts to version specifier")
				.parse()
				.expect("could not convert to number");
			let minor: u32 = it
				.next()
				.expect("not enough parts to version specifier")
				.parse()
				.expect("could not convert to number");

			Self::VersionPromotion { major, minor }
		} else {
			Self::Rename(value.to_owned())
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtensionDescriptor {
	/// Module path
	pub module: Vec<String>,
	/// Name used by vulkan when enabling extension.
	pub v_name: String,
	/// Numeric Id
	pub number: u32,
	/// Instance or Device Extension
	pub kind: ExtensionKind,
	/// Whether or not the extension is provisional/unstable
	pub provisional: bool,
	/// Extension dependencies
	pub depends: Vec<String>,
	/// The vulkan variant's this extension can be used with
	pub variant_compat: Vec<VulkanVariant>,
	/// The vulkan variant's this extensions has been ratified with
	pub ratified_variants: Vec<VulkanVariant>,
	/// Other extension or version to use instead
	pub deprecated_by: Option<ExtensionDeprecation>,
	/// Feture gates to put this extension behind
	pub feat_gates: Vec<String>,
}

impl From<&vk_parse::Extension> for ExtensionDescriptor {
	fn from(ext: &vk_parse::Extension) -> Self {
		// No longer used.
		assert_eq!(ext.requires, None);
		assert_eq!(ext.requires_core, None);
		assert_eq!(ext.protect, None);
		assert!(ext.name.starts_with("VK_"));
		{
			//Assure only one of these is specified. Spec is pretty inconsistent.
			let depr_count = ext.promotedto.is_some() as u8
				+ ext.deprecatedby.is_some() as u8
				+ ext.obsoletedby.is_some() as u8;
			assert!(depr_count <= 1);
		}

		let variant_compat: Vec<_> = ext
			.supported
			.as_ref()
			.expect("Supported is a required extension attribute")
			.split(',')
			.map(VulkanVariant::from)
			.collect();

		let ratified_variants: Vec<_> = ext
			.ratified
			.as_ref()
			.map(|v| v.split(',').map(VulkanVariant::from).collect())
			.unwrap_or_default();

		let mut extension_parts = ext.name.split_inclusive('_').skip(1);
		let module_group = extension_parts
			.next()
			.unwrap()
			.strip_suffix('_')
			.unwrap()
			.to_lowercase();
		let mut module_name = extension_parts.collect::<String>().to_lowercase();
		if module_name.starts_with(char::is_numeric) {
			//use n prefix for extension names starting with numbers because they are invalid for module names.
			module_name.insert_str(0, "n");
		}

		let number = ext.number.expect("Extension missing number") as u32;
		let kind: ExtensionKind = ext
			.ext_type
			.as_ref()
			.expect("Missing extension_type")
			.as_str()
			.try_into()
			.expect("Invalid extension_type");
		let deprecated_by = ext
			.promotedto
			.as_ref()
			.or(ext.deprecatedby.as_ref())
			.or(ext.obsoletedby.as_ref())
			.map(|s| s.as_str().into());

		let feat_gates: Vec<_> = ext
			.specialuse
			.as_ref()
			.map(|s| s.split(',').map(|s| s.to_owned() + "_exts").collect())
			.unwrap_or_default();

		ExtensionDescriptor {
			module: vec![module_group, module_name],
			v_name: ext.name.to_owned(),
			number,
			kind,
			provisional: ext.provisional,
			// TODO: parse ext.depends via ebnf library
			depends: vec![],
			variant_compat,
			ratified_variants,
			deprecated_by,
			feat_gates,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum VulkanVariant {
	Vulkan = 0,
	VulkanSC = 1,
}

impl From<&str> for VulkanVariant {
	fn from(value: &str) -> Self {
		match value.to_lowercase().as_str() {
			"vulkan" => Self::Vulkan,
			"vulkansc" => Self::VulkanSC,
			_ => panic!("Invalid extension variant"),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExtensionKind {
	Instance,
	Device,
}

impl From<&str> for ExtensionKind {
	fn from(value: &str) -> Self {
		match value.to_lowercase().as_str() {
			"instance" => Self::Instance,
			"device" => Self::Device,
			_ => panic!("Invalid extension kind"),
		}
	}
}
