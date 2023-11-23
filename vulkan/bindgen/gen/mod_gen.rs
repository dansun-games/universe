use std::fs::File;
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

pub fn write_struct(w: &mut impl io::Write, desc: &StructDescriptor) -> Result<(), std::io::Error> {
	writeln!(w, "#[repr(C)]")?;
	writeln!(w, "pub struct {} {{", desc.name)?;

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
