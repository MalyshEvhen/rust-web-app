// region:    --- Modules

mod error;
mod store;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		// FIXME - TBC
		Ok(ModelManager {})
	}
}
