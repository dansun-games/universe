use crate::descriptors::VarDescriptor;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CTypeInfo {
	c_type: &'static str,
	rust_type: &'static str,
}

pub static C_TYPE_MAPPINGS: &[CTypeInfo] = &[
	CTypeInfo {
		c_type: "void",
		rust_type: "c_void",
	},
	CTypeInfo {
		c_type: "char",
		rust_type: "u8",
	},
	CTypeInfo {
		c_type: "float",
		rust_type: "f32",
	},
	CTypeInfo {
		c_type: "double",
		rust_type: "f64",
	},
	CTypeInfo {
		c_type: "int8_t",
		rust_type: "i8",
	},
	CTypeInfo {
		c_type: "uint8_t",
		rust_type: "u8",
	},
	CTypeInfo {
		c_type: "int16_t",
		rust_type: "i16",
	},
	CTypeInfo {
		c_type: "uint16_t",
		rust_type: "u16",
	},
	CTypeInfo {
		c_type: "int32_t",
		rust_type: "i32",
	},
	CTypeInfo {
		c_type: "uint32_t",
		rust_type: "u32",
	},
	CTypeInfo {
		c_type: "int64_t",
		rust_type: "i64",
	},
	CTypeInfo {
		c_type: "uint64_t",
		rust_type: "u64",
	},
	CTypeInfo {
		c_type: "size_t",
		rust_type: "usize",
	},
];

//TODO: pointers and all the other stuff
pub fn convert_type(desc: &VarDescriptor) -> String {
	let rust_type = C_TYPE_MAPPINGS
		.iter()
		.find(|m| m.c_type == desc.c_type.as_str())
		.map(|v| v.rust_type.to_owned())
		.unwrap_or(desc.c_type.clone());

	return rust_type;
}
