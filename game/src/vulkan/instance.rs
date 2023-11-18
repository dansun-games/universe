use std::ffi::{c_char, CString};
use std::ops::Deref;

use ash::vk::{self, KhrPortabilityEnumerationFn};

use super::global::ENTRY;

pub struct Instance {
    inst: ash::Instance,
}

impl Drop for Instance {
    fn drop(&mut self) {
        println!("Instance being destroyed");
        unsafe { self.inst.destroy_instance(None) }
    }
}

//Not sure if I should impl DerefMut.
impl Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.inst
    }
}

impl Instance {
    pub fn new(layers: &[impl AsRef<str>], extensions: &[impl AsRef<str>]) -> Instance {
        /*
        We need two Vec for the cstring array because CString struct actually just contains an inner pointer to a byte slice.
        */
        let layers: Vec<CString> = layers
            .iter()
            .map(|l| CString::new(l.as_ref()).expect("CString conversion failure"))
            .collect();

        let char_ptr_layers: Vec<*const c_char> = layers.iter().map(|cstr| cstr.as_ptr()).collect();

        let create_info = vk::InstanceCreateInfo {
            enabled_layer_count: layers.len() as u32,
            pp_enabled_layer_names: char_ptr_layers.as_ptr(),
            ..Default::default()
        };

        vk::KhrExtension335Fn

        let inst = unsafe {
            ENTRY
                .create_instance(&create_info, None)
                .expect("Failed to create vulkan instance")
        };
        return Instance { inst };
    }

    pub fn destroy_instance(&self) {
        panic!("Do not call this directly. let Drop do the work.")
    }
}
