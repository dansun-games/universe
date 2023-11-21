#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CTypeInfo {
	c_type: &'static str,
	rust_type: &'static str,
}

pub static C_TYPE_MAPPINGS: &[CTypeInfo] = &[
	CTypeInfo {
		c_type: "VkSampleMask",
		rust_type: "u32",
	},
	CTypeInfo {
		c_type: "VkBool32",
		rust_type: "u32",
	},
	CTypeInfo {
		c_type: "VkFlags",
		rust_type: "u32",
	},
	CTypeInfo {
		c_type: "VkFlags64",
		rust_type: "u64",
	},
	CTypeInfo {
		c_type: "VkDeviceSize",
		rust_type: "u64",
	},
	CTypeInfo {
		c_type: "VkDeviceAddress",
		rust_type: "u64",
	},
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
