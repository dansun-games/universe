
pub struct SpecError {
	pub location: String,
	pub message: String,
}

impl SpecError {
    pub fn new(entity: &str, msg: &str) -> Self {
        Self {
            location: entity.to_owned(),
            message: msg.to_owned(),
        }
    }
}
