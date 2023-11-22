#![feature(lazy_cell)]
#![feature(result_option_inspect)]
#![feature(let_chains)]
#![feature(iter_intersperse)]
#![allow(unused_braces)]

mod descriptors;
mod gen;

use std::fs::File;
use std::path::Path;

use vk_parse as vk;

use descriptors::commands::get_commands;
use descriptors::constants::get_constants;
use descriptors::enums::{get_enum_aliases, get_enums};
use descriptors::extensions::get_extensions;
use descriptors::handles::get_handles;
use descriptors::platforms::get_platforms;
use descriptors::structs::get_structs;
use descriptors::unions::get_unions;
use gen::write_struct;


enum Descriptor {}

fn main() {
	println!("cargo:rerun-if-env-changed=GENERATE_BINDS");

	let vkxml = download_vk_xml().expect("Could not download vk.xml file.");
	let (registry, vk_errors) = vk::parse_stream(vkxml.as_bytes()).expect("Could not parse vk.xml");

	if vk_errors.len() > 0 {
		panic!("vk_parse errors: {:#?}", vk_errors);
	}


	/*
	---- type categories
	:done:
	struct
	union
	enum
	bitmask
	enum constants
	commands
	platforms

	:todo:
	include
	* basetype
	* funcpointer
	* formats
	* feature
	spirvextensions
	spirvcapabilities
	sync?
	* define - manual

	*/

	let types = all_types(&registry);

	let extensions = get_extensions(&registry);
	let (constants, const_aliases) = get_constants(&registry);
	let enums = get_enums(&registry);
	let enum_aliases = get_enum_aliases(&enums, &types);
	let unions = get_unions(&types);
	let (structs, struct_aliases) = get_structs(&types);
	let (commands, command_aliases) = get_commands(&registry);
	let platforms = get_platforms(&registry);
	let (handles, handle_aliases) = get_handles(&types);

	// let version_types = get_version_info(&registry);

	let path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("src")
		.join("structs.rs");
	println!("writing out at {:?}", path);

	let mut file = File::create(path).expect("Could not open file");

	for desc in structs {
		write_struct(&mut file, &desc).unwrap();
	}
}

fn all_types(reg: &vk::Registry) -> Vec<vk::Type> {
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
			vk_parse::TypesChild::Type(v) => Some(v.clone()),
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
		.collect()
}

fn download_vk_xml() -> Result<String, reqwest::Error> {
	use reqwest::blocking::Client;
	use reqwest::Method;

	const VK_XML_URL: &str =
		"https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml/vk.xml";

	let client = Client::builder().use_rustls_tls().build().unwrap();

	client.request(Method::GET, VK_XML_URL).send()?.text()
}
