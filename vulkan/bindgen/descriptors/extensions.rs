use std::rc::Rc;

use iter_ext::IterExt;
use vk_parse as vk;

use super::errors::SpecError;
use super::{Platform, Vendor};
use crate::util::NameMap;

pub fn extensions_from_registry(
	reg: &vk::Registry, vendors: &NameMap<Rc<Vendor>>, platforms: &NameMap<Rc<Platform>>,
) -> Result<NameMap<Extension>, SpecError> {
	
	let node = reg
		.0
		.iter()
		.find_single_map(|rc| match rc {
			vk::RegistryChild::Extensions(exts) => Some(exts),
			_ => None,
		})
		.map_err(|err| SpecError::new("extensions", &err.to_string()))?;

	let extensions: NameMap<_> = node
		.children
		.iter()
		.map(|def| Extension::from_vk_def(def, vendors, platforms))
		.map(|v| (v.name.clone(), v))
		.collect();

	Ok(extensions)
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
pub struct Extension {
	/// Module path
	pub module: Vec<String>,
	/// Name used by vulkan when enabling extension.
	pub name: String,
	/// Instance or Device Extension
	pub kind: ExtensionKind,
	/// Numeric Id
	pub number: u32,
	// Vendor
	pub vendor: Rc<Vendor>,
	/// Special use flag to put this extension behind
	pub special_use: Vec<SpecialUse>,
	// Platform
	pub platform: Option<Rc<Platform>>,
	/// Whether or not the extension is provisional/unstable
	pub provisional: bool,
	/// Extension dependencies
	pub depends: Vec<Rc<Extension>>,
	/// The vulkan variant's this extension can be used with
	pub variant_compat: Vec<VulkanVariant>,
	/// The vulkan variant's this extensions has been ratified with
	pub ratified_variants: Vec<VulkanVariant>,
	/// Other extension or version to use instead
	pub deprecated_by: Option<ExtensionDeprecation>,
}

impl Extension {
	fn from_vk_def(
		def: &vk::Extension, vendors: &NameMap<Rc<Vendor>>, platforms: &NameMap<Rc<Platform>>,
	) -> Self {
		// No longer used.
		assert_eq!(def.requires, None);
		assert_eq!(def.requires_core, None);
		assert_eq!(def.protect, None);
		assert!(def.name.starts_with("VK_"));

		//Assert only one of these is specified. Spec is pretty inconsistent.
		{
			let depr_count = def.promotedto.is_some() as u8
				+ def.deprecatedby.is_some() as u8
				+ def.obsoletedby.is_some() as u8;
			assert!(depr_count <= 1);
		}
		let deprecated_by = def
			.promotedto
			.as_ref()
			.or(def.deprecatedby.as_ref())
			.or(def.obsoletedby.as_ref())
			.map(|s| ExtensionDeprecation::from(s.as_str()));

		let variant_compat: Vec<_> = def
			.supported
			.as_ref()
			.expect("Supported is a required extension attribute")
			.split(',')
			.map(VulkanVariant::from)
			.collect();

		let ratified_variants: Vec<_> = def
			.ratified
			.as_ref()
			.map(|v| v.split(',').map(VulkanVariant::from).collect())
			.unwrap_or_default();

		let mut extension_parts = def.name.split_inclusive('_').skip(1);
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

		let number = def.number.expect("Extension missing number") as u32;
		let kind: ExtensionKind = def
			.ext_type
			.as_ref()
			.expect("Missing extension_type")
			.as_str()
			.try_into()
			.expect("Invalid extension_type");

		let special_use: Vec<_> = def
			.specialuse
			.as_ref()
			.map(|s| s.split(',').map(SpecialUse::from).collect())
			.unwrap_or_default();

		let vendor_name = def.author.as_ref().expect("Missing vendor");
		let vendor = vendors.get(vendor_name).expect("Unknown vendor name").clone();
		let platform = def
			.platform
			.as_ref()
			.map(|name| platforms.get(name).expect("Unknown platform name").clone());

		Extension {
			module: vec![module_group, module_name],
			name: def.name.to_owned(),
			number,
			vendor,
			platform,
			kind,
			provisional: def.provisional,
			// TODO: parse ext.depends via ebnf library
			depends: vec![],
			variant_compat,
			ratified_variants,
			deprecated_by,
			special_use,
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

#[derive(Debug, PartialEq, Eq)]
pub enum SpecialUse {
	DebuggingTools,
	DeveloperTools,
	GLEmulation,
	D3DEmulation,
	CADSupport,
}

impl From<&str> for SpecialUse {
	fn from(value: &str) -> Self {
		match value.to_lowercase().as_str() {
			"debugging" => Self::DebuggingTools,
			"devtools" => Self::DeveloperTools,
			"glemulation" => Self::GLEmulation,
			"d3demulation" => Self::D3DEmulation,
			"cadsupport" => Self::CADSupport,
			_ => panic!("Invalid special use kind"),
		}
	}
}
