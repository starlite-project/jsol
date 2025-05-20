mod op;

use std::num::NonZeroUsize;

pub use self::op::Operation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsolModule {
	operations: Vec<Operation>,
	id: ModuleId,
}

impl JsolModule {
	pub(crate) const fn new(id: ModuleId) -> Self {
		Self {
			operations: Vec::new(),
			id,
		}
	}

	#[must_use]
	pub const fn operations(&self) -> &Vec<Operation> {
		&self.operations
	}

	pub const fn operations_mut(&mut self) -> &mut Vec<Operation> {
		&mut self.operations
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ModuleId(NonZeroUsize);
