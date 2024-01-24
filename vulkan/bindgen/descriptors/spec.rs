use iter_ext::IterExt;
use vk::Extension;
use vk_parse as vk;

use super::errors::SpecError;
use super::{get_extensions, Platform, Vendor};
use crate::util::NameMap;

struct Spec {
	platforms: NameMap<Platform>,
	vendors: NameMap<Vendor>,
    extensions: NameMap<Extension>,

}

impl TryFrom<&vk::Registry> for Spec {
	type Error = SpecError;

	fn try_from(reg: &vk::Registry) -> Result<Self, Self::Error> {
		let vendors: NameMap<Vendor> = reg
            .0
			.iter()
			.filter_map(|rc| match rc {
				vk::RegistryChild::Tags(t) => Some(t),
				_ => None,
			})
			.single()
			.map_err(|err| SpecError::new("vendors", &err.to_string()))?
			.children
			.iter()
			.map(Vendor::from)
			.map(|v| (v.name.clone(), v))
			.collect();

		let platforms: NameMap<Platform> = reg
            .0
			.iter()
			.filter_map(|rc| match rc {
				vk::RegistryChild::Platforms(p) => Some(p),
				_ => None,
			})
			.single()
			.map_err(|err| SpecError::new("platforms", &err.to_string()))?
			.children
			.iter()
            .filter(|p| p.name != "provisional") //provisional doesnt follow regular platform conventions
			.map(Platform::try_from)
			.map(|r| r.map(|plat| (plat.name.clone(), plat)))
			.try_collect()?;

        //Validate vendor references are valid.
		for plat in platforms.values() {
			if vendors.contains_key(&plat.vendor) {
				let entity = format!("platforms/{}", plat.name);
				let msg = format!("Unknown vendor {}", plat.vendor);
				return Err(SpecError::new(&entity, &msg));
			}
		}

        let extensions = get_extensions(&reg);



		Ok(Self { platforms, vendors })
	}
}
