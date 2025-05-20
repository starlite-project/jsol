#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod module;
mod ops;

use jsol_parse::RawJsolModule;

pub use self::{module::Module, ops::Operation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkContext {
	modules: Vec<Module>,
}

impl LinkContext {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			modules: Vec::new(),
		}
	}

	pub fn resolve_module(&mut self, module: RawJsolModule) {
		let mut resolved = Module::new();

		if let Some(ops) = module.operations() {
			for op in ops.iter().cloned() {
				resolved
					.operations_mut()
					.push(Operation::resolve_operation(self, op));
			}
		}

		self.modules.push(resolved);
	}
}

impl Default for LinkContext {
	fn default() -> Self {
		Self::new()
	}
}
