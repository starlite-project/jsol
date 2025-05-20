use crate::Operation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
	operations: Vec<Operation>,
}

impl Module {
	pub(crate) const fn new() -> Self {
		Self {
			operations: Vec::new(),
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
