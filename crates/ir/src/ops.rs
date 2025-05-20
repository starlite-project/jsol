use jsol_parse::RawOperation;

use crate::LinkContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
	Nop,
}

impl Operation {
	#[must_use]
	pub fn resolve_operation(_ctx: &LinkContext, raw_op: RawOperation) -> Self {
		match raw_op {
			RawOperation::Nop => Self::Nop,
		}
	}
}
