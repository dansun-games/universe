use std::io::Write;

use crate::descriptors::structs::StructDescriptor;

mod str_convert;

use str_convert::*;

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

		writeln!(w, "{}: bool,", member_name)?;
	}

	writeln!(w, "}}")?;
	writeln!(w)?;

	Ok(())
}
