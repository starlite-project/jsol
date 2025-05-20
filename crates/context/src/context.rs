use jsol_parse::RawJsolFile;

use super::ir::JsolModule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsolContext {
	modules: Vec<JsolModule>,
}

impl JsolContext {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			modules: Vec::new(),
		}
	}
}

impl Default for JsolContext {
	fn default() -> Self {
		Self::new()
	}
}
