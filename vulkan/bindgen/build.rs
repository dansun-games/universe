#![feature(lazy_cell)]
#![feature(result_option_inspect)]
#![feature(let_chains)]
#![feature(iter_intersperse)]
#![allow(unused_braces)]

mod descriptors;
mod gen;

use std::path::PathBuf;

use descriptors::{
	get_commands, get_constants, get_enum_aliases, get_enums, get_extensions, get_handles,
	get_platforms, get_structs, get_type_aliases, get_unions,
};
use gen::ModGen;
use vk_parse as vk;


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
	basetype

	:todo:
	include
	* funcpointer
	* formats
	* feature
	spirvextensions
	spirvcapabilities
	sync?
	* define - manual

	*/


	println!("{:#?}", get_enums(&registry));

	let types = all_types(&registry);

	let extensions = get_extensions(&registry);
	let platforms = get_platforms(&registry);

	let type_aliases = get_type_aliases();
	let (constants, const_aliases) = get_constants(&registry);
	let enums = get_enums(&registry);
	let enum_aliases = get_enum_aliases(&enums, &types);
	let unions = get_unions(&types);
	let (handles, handle_aliases) = get_handles(&types);
	let (structs, struct_aliases) = get_structs(&types);
	let (commands, command_aliases) = get_commands(&registry);

	// let version_types = get_version_info(&registry);

	//cwd is Cargo.toml dir
	let root_path = PathBuf::from("./src/");

	//Right now, we are just shoving everything into a single file.
	//In the future we should break them up in a more category-by-use-case way.
	//going to use some kind of toml file to automate the process.
	let root_file = ModGen {
		name: String::from("everything"),
		module_doc: None,
		constants,
		const_aliases,
		enums,
		enum_aliases,
		unions,
		structs,
		struct_aliases,
		commands,
		command_aliases,
		handles,
		handle_aliases,
		type_aliases,
	};

	root_file.generate(&root_path).unwrap();
}

fn all_types(reg: &vk::Registry) -> Vec<vk::Type> {
	// Josh - convert to map
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
