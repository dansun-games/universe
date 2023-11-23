use std::iter::repeat;
use std::{fs::File, fmt::format};
use std::io;
use std::path::PathBuf;

use super::str_convert::convert_identifier;
use super::type_convert::convert_type;
use crate::descriptors::{
	Alias, CommandDescriptor, ConstDescriptor, EnumDescriptor, HandleDescriptor, StructDescriptor,
	UnionDescriptor,
};

pub struct ModGen {
	pub name: String,
	pub module_doc: Option<String>,
	pub content: FileContent,
}

impl ModGen {
	pub fn generate(&self, root: &PathBuf) -> Result<(), io::Error> {
		let file_name = format!("{}.rs", self.name);
		let out = root.join(file_name.as_str());
		let mut file = File::create(out)?;

		for desc in &self.content.type_aliases {
			write_type_alias(&mut file, desc)?;
		}

		for desc in &self.content.handles {
			write_handle(&mut file, desc)?;
		}

		for desc in &self.content.enums {
			write_enum(&mut file, desc)?;
		}

		for desc in &self.content.structs {
			write_struct(&mut file, desc)?;
		}

		Ok(())
	}
}


pub struct FileContent {
	pub constants: Vec<ConstDescriptor>,
	pub const_aliases: Vec<Alias>,
	pub enums: Vec<EnumDescriptor>,
	pub enum_aliases: Vec<Alias>,
	pub unions: Vec<UnionDescriptor>,
	pub structs: Vec<StructDescriptor>,
	pub struct_aliases: Vec<Alias>,
	pub commands: Vec<CommandDescriptor>,
	pub command_aliases: Vec<Alias>,
	pub handles: Vec<HandleDescriptor>,
	pub handle_aliases: Vec<Alias>,
	pub type_aliases: Vec<Alias>,
}

fn strip_vk(name: &str) -> &str {
	name.strip_prefix("Vk").expect("missing vk prefix")
}

fn write_struct(w: &mut impl io::Write, desc: &StructDescriptor) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "#[repr(C)]")?;
	writeln!(w, "pub struct {} {{", name)?;

	for mem in &desc.members {
		if mem.deprecated {
			writeln!(w, "#[deprecated(note = \"Ignored\")]")?;
		}
		if let Some(variant) = &mem.variant_feat {
			writeln!(w, "#[cfg({variant})]")?;
		}

		let member_name = convert_identifier(mem.var_spec.name.as_str());
		let rtype = convert_type(&mem.var_spec);

		writeln!(w, "{}: {},", member_name, rtype)?;
	}

	writeln!(w, "}}")?;
	writeln!(w)?;

	Ok(())
}

fn write_type_alias(w: &mut impl io::Write, desc: &Alias) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "#[repr(transparent)]")?;
	writeln!(w, "pub struct {}({});", name, desc.alias_for)?;
	writeln!(w)?;

	Ok(())
}

fn write_enum(w: &mut impl io::Write, desc: &EnumDescriptor) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "#[repr(u{})]", desc.bit_width)?;
	writeln!(w, "pub enum {} {{", name)?;

	for val in &desc.values {
		if desc.is_bitmask {
			// let value = format!("{:b}", val.value);
			// assert!(value.len() < desc.bit_width);
			// let pad: String = repeat('0').take(desc.bit_width - value.len()).collect();
			writeln!(w, "{} = {:#b},", val.name, val.value)?;
		} else {
			writeln!(w, "{} = {},", val.name, val.value)?;
		}
	}

	writeln!(w, "}}")?;
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