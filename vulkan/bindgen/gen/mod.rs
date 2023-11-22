use std::io::Write;

use crate::descriptors::structs::StructDescriptor;

mod str_convert;
mod type_convert;

use str_convert::*;

use self::type_convert::convert_type;

pub fn write_struct(w: &mut impl Write, desc: &StructDescriptor) -> Result<(), std::io::Error> {
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
