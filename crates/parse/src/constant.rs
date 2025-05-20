use jsol_value::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawConstant {
	name: String,
	value: Value,
}

impl RawConstant {
	#[must_use]
	pub const fn new(name: String, value: Value) -> Self {
		Self { name, value }
	}
}
