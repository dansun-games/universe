use std::{sync::LazyLock};

use ash::{
    vk::{
        api_version_major, api_version_minor, api_version_patch, api_version_variant,
        API_VERSION_1_0, make_api_version, LayerProperties,
    },
    Entry,
};

pub struct VulkanVersion {
    pub variant: u32,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl From<u32> for VulkanVersion {
    fn from(value: u32) -> Self {
        Self {
            variant: api_version_variant(value),
            major: api_version_major(value),
            minor: api_version_minor(value),
            patch: api_version_patch(value),
        }
    }
}

impl Into<u32> for VulkanVersion {
    fn into(self) -> u32 {
        make_api_version(self.variant, self.major, self.minor, self.patch)
    }
}

//Will have to rethink this pattern if we ever want to gracefully handle vulkan linking failure
pub(super) static ENTRY: LazyLock<Entry> =
    LazyLock::new(|| unsafe { Entry::load().expect("could not link with vulkan") });

pub static VULKAN_VERSION: LazyLock<VulkanVersion> = LazyLock::new(|| {
    ENTRY
        .try_enumerate_instance_version()
        .expect("Could not get vulkan version")
        .unwrap_or(API_VERSION_1_0)
        .into()
});

pub fn layers() -> Vec<LayerProperties> {
    ENTRY.enumerate_instance_layer_properties().unwrap()
}