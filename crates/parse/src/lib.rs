#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod constant;

use serde::{Deserialize, Serialize};

pub use self::constant::RawConstant;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RawJsolModule {
	Entrypoint {
		#[serde(default)]
		operations: Vec<RawOperation>,
		#[serde(default)]
		constants: Vec<RawConstant>,
	},
	Module {
		#[serde(skip_serializing_if = "Option::is_none", default)]
		operations: Option<Vec<RawOperation>>,
		#[serde(skip_serializing_if = "Vec::is_empty", default)]
		constants: Vec<RawConstant>,
	},
}

impl RawJsolModule {
	#[must_use]
	pub fn operations(&self) -> Option<&[RawOperation]> {
		match self {
			Self::Entrypoint { operations, .. } => Some(operations),
			Self::Module { operations, .. } => Some(operations.as_ref()?),
		}
	}

	pub fn operations_mut(&mut self) -> Option<&mut Vec<RawOperation>> {
		match self {
			Self::Entrypoint { operations, .. } => Some(operations),
			Self::Module { operations, .. } => Some(operations.as_mut()?),
		}
	}

	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn constants(&self) -> &[RawConstant] {
		match self {
			Self::Module { constants, .. } | Self::Entrypoint { constants, .. } => constants,
		}
	}

	#[allow(clippy::missing_const_for_fn)]
	pub fn constants_mut(&mut self) -> &mut Vec<RawConstant> {
		match self {
			Self::Module { constants, .. } | Self::Entrypoint { constants, .. } => constants,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RawOperation {
	Nop,
}
