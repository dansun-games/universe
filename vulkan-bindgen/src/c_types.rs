
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CTypeInfo {
    c_type: &'static str,
    rust_type: &'static str,
}

static c_type_mappings: &[CTypeInfo] = &[
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
        rust_type: "void",
    },
    CTypeInfo {
        c_type: "char",
        rust_type: "c_char",
    },
    CTypeInfo {
        c_type: "float",
        rust_type: "c_float",
    },
    CTypeInfo {
        c_type: "double",
        rust_type: "c_double",
    },
    CTypeInfo {
        c_type: "int8_t",
        rust_type: "c_ushort"
    },
    CTypeInfo {
        c_type: "uint8_t",
        rust_type: "c_char",
    },
    CTypeInfo {
        c_type: "int16_t",
        rust_type: "todo",
    },
    CTypeInfo {
        c_type: "uint16_t",
        rust_type: "todo",
    },
    CTypeInfo {
        c_type: "uint32_t",
        rust_type: "todo",
    },
    CTypeInfo {
        c_type: "uint64_t",
        rust_type: "todo",
    },
    CTypeInfo {
        c_type: "int32_t",
        rust_type: "todo",
    },
    CTypeInfo {
        c_type: "int64_t",
        rust_type: "todo",
    },
    CTypeInfo {
        c_type: "size_t",
        rust_type: "todo",
    },
];