use core::panic;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use case_style::CaseStyle;

use super::str_convert::*;
use super::type_convert::{convert_type, USIZE_CONSTANTS};
use crate::descriptors::{
	Alias, CommandDescriptor, ConstDescriptor, EnumDescriptor, HandleDescriptor, StructDescriptor,
	UnionDescriptor, BitflagDescriptor,
};
use crate::gen::type_convert::{convert_const_value, C_TYPE_MAPPINGS};
use crate::util::NameMap;

static RESERVED_IDENT: &[&str] = &["type"];

pub struct ModGen {
	pub name: String,
	pub module_doc: Option<String>,

	//Content
	pub type_aliases: NameMap<Alias>,
	pub constants: NameMap<ConstDescriptor>,
	pub const_aliases: NameMap<Alias>,
	pub handles: NameMap<HandleDescriptor>,
	pub handle_aliases: NameMap<Alias>,
	pub enums: NameMap<EnumDescriptor>,
	pub enum_aliases: NameMap<Alias>,
	pub bitflags: NameMap<BitflagDescriptor>,
	pub bitflag_aliases: NameMap<Alias>,
	pub unions: NameMap<UnionDescriptor>,
	pub union_aliases: NameMap<Alias>,
	pub structs: NameMap<StructDescriptor>,
	pub struct_aliases: NameMap<Alias>,
	pub commands: NameMap<CommandDescriptor>,
	pub command_aliases: NameMap<Alias>,
}

impl ModGen {
	pub fn generate(&self, root: &PathBuf) -> Result<(), io::Error> {
		let file_name = format!("{}.rs", self.name);
		let out = root.join(file_name.as_str());
		let mut file = File::create(out)?;

		write_header(&mut file)?;

		for desc in self.type_aliases.values() {
			write_type_alias(&mut file, desc)?;
		}

		for desc in self.constants.values() {
			write_const(&mut file, desc)?;
		}

		for desc in self.const_aliases.values() {
			//this is pretty clunky right now because we have vec instead of hashmap
			let alias_for = &self.constants[&desc.alias_for];
			write_const_alias(&mut file, desc)?;
		}

		for desc in self.handles.values() {
			write_handle(&mut file, desc)?;
		}

		for desc in self.enums.values() {
			write_enum(&mut file, desc)?;
		}

		for desc in self.bitflags.values() {
			write_bitflags(&mut file, desc)?;
		}

		for desc in self.bitflag_aliases.values() {
			write_bitflag_aliases(&mut file, desc)?;
		}

		for desc in self.structs.values() {
			write_struct(&mut file, desc)?;
		}

		Ok(())
	}
}

fn write_header(w: &mut impl io::Write) -> Result<(), io::Error> {
	writeln!(w, "use std::ffi::c_void;")?;
	writeln!(w, "use bitflags::bitflags;")?;
	writeln!(w)?;

	Ok(())
}

fn write_struct(w: &mut impl io::Write, desc: &StructDescriptor) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "#[repr(C)]")?;
	writeln!(w, "pub struct {} {{", name)?;

	for mem in &desc.members {
		if mem.deprecated {
			writeln!(w, "\t#[deprecated(note = \"Ignored\")]")?;
		}
		if let Some(variant) = &mem.variant_feat {
			writeln!(w, "\t#[cfg({variant})]")?;
		}

		let mut member_name = fix_pascal(&mem.var_spec.name);
		member_name = CaseStyle::from_camelcase(&member_name).to_snakecase();
		if RESERVED_IDENT.contains(&member_name.as_str()) {
			member_name.insert_str(0, "r#");
		}
		let rtype = convert_type(&mem.var_spec);

		writeln!(w, "\t{}: {},", member_name, rtype)?;
	}

	writeln!(w, "}}")?;
	writeln!(w)?;

	Ok(())
}

fn write_const(w: &mut impl io::Write, desc: &ConstDescriptor) -> Result<(), io::Error> {
	let name = convert_const_name(&desc.name);
	let value = convert_const_value(&desc.value);
	let rust_type = {
		if USIZE_CONSTANTS.contains(&name.as_str()) {
			"usize"
		} else if name == "TRUE" || name == "FALSE" {
			"Bool32"
		} else {
			C_TYPE_MAPPINGS
				.iter()
				.find(|m| m.c_type == desc.c_type)
				.expect("Could not convert c_type for const")
				.rust_type
		}
	};

	writeln!(w, "const {}: {} = {};", name, rust_type, value)?;
	writeln!(w)?;

	Ok(())
}

fn write_const_alias(w: &mut impl io::Write, desc: &Alias) -> Result<(), io::Error> {
	let name = convert_const_name(&desc.name);
	let alias_for = desc.alias_for.trim_start_matches("VK_");

	//we dont have a way to know the rust type right now.. need hashmap for const so we can lookup the correct type
	let rust_type = "TODO";

	writeln!(w, "//const {}: {} = {};", name, rust_type, alias_for)?;
	writeln!(w)?;

	Ok(())
}

fn write_type_alias(w: &mut impl io::Write, desc: &Alias) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "pub type {} = {};", name, desc.alias_for)?;
	writeln!(w)?;

	Ok(())
}

fn write_enum(w: &mut impl io::Write, desc: &EnumDescriptor) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "#[repr(C)]")?;
	writeln!(w, "pub enum {name} {{")?;

	for val in &desc.values {
		let name = convert_enum_val_name(&val.name);

		writeln!(w, "\t{} = {},", name, val.value)?;
	}

	writeln!(w, "}}")?;
	writeln!(w)?;

	Ok(())
}

fn write_bitflags(w: &mut impl io::Write, desc: &BitflagDescriptor) -> Result<(), io::Error> {
	let name = strip_vk(&desc.name);
	let bits_name = desc.bits_name.as_ref().map(|v| strip_vk(v));
	let bit_width = desc.bit_width;
	let rust_type = match bit_width {
		32 => "Flags",
		64 => "Flags64",
		_ => panic!("Invalid bit_width for flags")
	};

	if let Some(bits_name) = bits_name && !desc.values.is_empty() {
		println!("{bits_name}");
		let bits_prefix = extract_bits_prefix(bits_name);

		writeln!(w, "#[repr(u{bit_width})]")?;
		writeln!(w, "pub enum {bits_name} {{")?;
		for bit in &desc.values {
			let mut bit_name = convert_enum_val_name(&bit.name);
			writeln!(w, "\t{bit_name} = 1 << {},", bit.bitpos)?;
		}
		writeln!(w, "}}")?;
		writeln!(w)?;

		if !desc.aliases.is_empty() {
			writeln!(w, "impl {bits_name} {{")?;
			for flag in &desc.aliases {
				let flag_name = convert_enum_val_name(&flag.name);
				writeln!(w, "\tconst {flag_name}: u{bit_width} = Self::{flag_name};")?;
			}
			writeln!(w, "}}")?;
			writeln!(w)?;
		}
	}

	writeln!(w, "bitflags! {{")?;
	writeln!(w, "\t#[repr(transparent)]")?;
	writeln!(w, "\tpub struct {}: {} {{", name, rust_type)?;
	for bit in &desc.values {
		let bits_name = bits_name.expect("values defined without bits");
		let flag_name = convert_const_name(&bit.name);
		let bit_name = convert_enum_val_name(&bit.name);
		writeln!(w, "\t\tconst {flag_name} = {bits_name}::{bit_name} as {rust_type};")?;
	}
	// https://docs.rs/bitflags/latest/bitflags/#externally-defined-flags
	// writeln!(w, "\t\tconst _ = !0")?;

	writeln!(w, "\t}}")?;
	writeln!(w, "}}")?;
	writeln!(w)?;

	Ok(())
}

fn write_bitflag_aliases(w: &mut impl io::Write, desc: &Alias) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());
	let alias_for = strip_vk(&desc.alias_for);
	
	writeln!(w, "pub type {name} = {alias_for};")?;
	writeln!(w)?;

	Ok(())
}

fn write_handle(w: &mut impl io::Write, desc: &HandleDescriptor) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "#[repr(transparent)]")?;
	writeln!(w, "pub struct {}({});", name, desc.rust_type)?;
	writeln!(w)?;

	Ok(())
}


fn extract_bits_prefix(str: &str) -> &str {
	let i = str.find("FlagBits").expect("Missing FlagBits suffix");
	&str[..i]
}
