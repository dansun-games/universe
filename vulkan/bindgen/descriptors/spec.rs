use std::rc::Rc;

use vk_parse as vk;

use super::errors::SpecError;
use super::{Extension, Platform, Vendor};
use crate::descriptors::{
	extensions_from_registry, platforms_from_registry, vendors_from_registry,
};
use crate::util::NameMap;

pub struct Spec {
	pub platforms: NameMap<Rc<Platform>>,
	pub vendors: NameMap<Rc<Vendor>>,
	pub extensions: NameMap<Rc<Extension>>,
}

impl TryFrom<&vk::Registry> for Spec {
	type Error = SpecError;

	fn try_from(reg: &vk::Registry) -> Result<Self, Self::Error> {
		let platforms = platforms_from_registry(reg)?;
		let platforms: NameMap<_> = platforms.into_iter().map(|(n, plat)| (n, Rc::new(plat))).collect();
		
		let vendors = vendors_from_registry(reg)?;
		let vendors: NameMap<_> = vendors.into_iter().map(|(n, ven)| (n, Rc::new(ven))).collect();

		let extensions = extensions_from_registry(reg, &vendors, &platforms)?;
		let extensions: NameMap<_> = extensions.into_iter().map(|(n, ext)| (n, Rc::new(ext))).collect();

		Ok(Self {
			platforms,
			vendors,
			extensions,
		})
	}
}
