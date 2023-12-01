

use std::error::Error;

use ash::Instance;
use game::vulkan;

fn main() -> Result<(), Box<dyn Error>> {
    let version = vulkan::VULKAN_VERSION;

    println!("Variant: {}", version.variant);
    println!(
        "Version: {}.{}.{}",
        version.major, version.minor, version.patch
    );

    let available_layers = global_layers();
    println!("{:?}", available_layers);

    let instance = Instance::new(&["test"]);

    let devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("could not enumerate physical devices")
    };

    let device_desc: Vec<_> = devices
        .into_iter()
        .map(|dev| unsafe { instance.get_physical_device_properties(dev) })
        .collect();

    println!("{:#?}", device_desc);

    return Ok(());
}
