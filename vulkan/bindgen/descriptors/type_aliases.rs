use super::alias::Alias;

pub fn get_type_aliases() -> Vec<Alias> {
    //These are hardcoded right now because some have some annoying definitions.
    vec![
        Alias {
            name: String::from("VkSampleMask"),
            alias_for: String::from("u32"),
        },
        Alias {
            name: String::from("VkBool32"),
            alias_for: String::from("u32"),
        },
        Alias {
            name: String::from("VkFlags"),
            alias_for: String::from("u32"),
        },
        Alias {
            name: String::from("VkFlags64"),
            alias_for: String::from("u64"),
        },
        Alias {
            name: String::from("VkDeviceSize"),
            alias_for: String::from("u64"),
        },
        Alias {
            name: String::from("VkDeviceAddress"),
            alias_for: String::from("u64"),
        },
    ]
}