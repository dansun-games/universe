use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use case_style::CaseStyle;

use super::str_convert::{fix_pascal, strip_vk};
use super::type_convert::convert_type;
use crate::descriptors::{
	Alias, CommandDescriptor, ConstDescriptor, EnumDescriptor, HandleDescriptor, StructDescriptor,
	UnionDescriptor, VarDescriptor,
};
use crate::gen::type_convert::{convert_const_value, C_TYPE_MAPPINGS};

static RESERVED_IDENT: &[&str] = &["type"];

pub struct ModGen {
	pub name: String,
	pub module_doc: Option<String>,

	//Content
	pub type_aliases: HashMap<String, Alias>,
	pub constants: HashMap<String, ConstDescriptor>,
	pub handles: HashMap<String, HandleDescriptor>,
	pub handle_aliases: HashMap<String, Alias>,
	pub const_aliases: HashMap<String, Alias>,
	pub enums: HashMap<String, EnumDescriptor>,
	pub enum_aliases: HashMap<String, Alias>,
	pub unions: HashMap<String, UnionDescriptor>,
	pub structs: HashMap<String, StructDescriptor>,
	pub struct_aliases: HashMap<String, Alias>,
	pub commands: HashMap<String, CommandDescriptor>,
	pub command_aliases: HashMap<String, Alias>,
}

impl ModGen {
	pub fn generate(&self, root: &PathBuf) -> Result<(), io::Error> {
		let file_name = format!("{}.rs", self.name);
		let out = root.join(file_name.as_str());
		let mut file = File::create(out)?;

		for desc in self.type_aliases.values() {
			write_type_wrapper(&mut file, desc)?;
		}

		for desc in self.constants.values() {
			write_const(&mut file, desc)?;
		}

		for desc in self.const_aliases.values() {
			write_const_alias(&mut file, desc)?;
		}

		for desc in self.handles.values() {
			write_handle(&mut file, desc)?;
		}

		for desc in self.enums.values() {
			write_enum(&mut file, desc)?;
		}

		for desc in self.structs.values() {
			write_struct(&mut file, desc)?;
		}

		Ok(())
	}
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

		let mut member_name = fix_pascal(&mem.var_spec.name);
		member_name = CaseStyle::from_camelcase(&member_name).to_snakecase();
		if RESERVED_IDENT.contains(&member_name.as_str()) {
			member_name.insert_str(0, "r#");
		}
		let rtype = convert_type(&mem.var_spec);

		writeln!(w, "{}: {},", member_name, rtype)?;
	}

	writeln!(w, "}}")?;
	writeln!(w)?;

	Ok(())
}

fn write_const(w: &mut impl io::Write, desc: &ConstDescriptor) -> Result<(), io::Error> {
	let name = desc.name.trim_start_matches("VK_");
	let value = convert_const_value(&desc.value);
	let rust_type = {
		if name == "TRUE" || name == "FALSE" {
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
	let name = desc.name.trim_start_matches("VK_");
	let alias_for = desc.alias_for.trim_start_matches("VK_");

	writeln!(w, "const {} = {};", name, alias_for)?;
	writeln!(w)?;

	Ok(())
}

fn write_type_wrapper(w: &mut impl io::Write, desc: &Alias) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	writeln!(w, "#[repr(transparent)]")?;
	writeln!(w, "pub struct {}({});", name, desc.alias_for)?;
	writeln!(w)?;

	Ok(())
}

fn write_enum(w: &mut impl io::Write, desc: &EnumDescriptor) -> Result<(), io::Error> {
	let name = strip_vk(desc.name.as_ref());

	let repr = if desc.is_bitmask {
		format!("u{}", desc.bit_width)
	} else {
		//let rust figure it out...
		String::from("C")
	};

	writeln!(w, "#[repr({repr})]")?;
	writeln!(w, "pub enum {name} {{")?;

	for val in &desc.values {
		let name = CaseStyle::from_snakecase(&val.name).to_pascalcase();
		let name = strip_vk(&name);
		let value = if desc.is_bitmask {
			format!("{:#b}", val.value)
		} else {
			format!("{}", val.value)
		};

		writeln!(w, "{} = {},", name, value)?;
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
