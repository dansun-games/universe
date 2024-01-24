use vk_parse as vk;


pub struct Vendor {
    pub name: String
}


impl From<&vk::Tag> for Vendor {
	fn from(tag: &vk::Tag) -> Self {
		let name = tag.name.clone();
        //only really care about name.
		Self { name }
	}
}
