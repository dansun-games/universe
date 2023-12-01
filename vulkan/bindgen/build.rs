#![feature(lazy_cell)]
#![feature(let_chains)]
#![feature(iter_intersperse)]
#![allow(unused_braces)]

mod descriptors;
mod gen;
mod util;

use std::path::PathBuf;

use descriptors::{
	get_commands, get_constants, get_enums, get_extensions, get_handles,
	get_platforms, get_structs, get_type_aliases, get_unions,
};
use gen::ModGen;
use vk_parse as vk;

use crate::descriptors::get_bitflags;


fn main() {
	println!("cargo:rerun-if-env-changed=GENERATE_BINDS");

	let vkxml = download_vk_xml().expect("Could not download vk.xml file.");
	let (reg, vk_errors) = vk::parse_stream(vkxml.as_bytes()).expect("Could not parse vk.xml");

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


	let extensions = get_extensions(&reg);
	let platforms = get_platforms(&reg);

	let type_aliases = get_type_aliases();
	let (constants, const_aliases) = get_constants(&reg);
	let (enums, enum_aliases) = get_enums(&reg);
	let (bitmasks, bitmask_aliases) = get_bitflags(&reg);
	let (unions, union_aliases) = get_unions(&reg);
	let (handles, handle_aliases) = get_handles(&reg);
	let (structs, struct_aliases) = get_structs(&reg);
	let (commands, command_aliases) = get_commands(&reg);

	// let version_types = get_version_info(&reg);

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
		bitflags: bitmasks,
		bitflag_aliases: bitmask_aliases,
		unions,
		union_aliases,
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

fn download_vk_xml() -> Result<String, reqwest::Error> {
	use reqwest::blocking::Client;
	use reqwest::Method;

	const VK_XML_URL: &str =
		"https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml/vk.xml";

	let client = Client::builder().use_rustls_tls().build().unwrap();

	client.request(Method::GET, VK_XML_URL).send()?.text()
}
