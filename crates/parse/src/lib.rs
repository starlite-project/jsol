#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RawJsolModule {
	Entrypoint {
		operations: Vec<RawOperation>,
	},
	Module {
		#[serde(skip_serializing_if = "Vec::is_empty", default)]
		exports: Vec<()>,
		#[serde(skip_serializing_if = "Option::is_none", default)]
		operations: Option<Vec<RawOperation>>,
	},
}

impl RawJsolModule {
	#[must_use]
	pub const fn entrypoint() -> Self {
		Self::Entrypoint {
			operations: Vec::new(),
		}
	}

	#[must_use]
	pub const fn module() -> Self {
		Self::Module {
			exports: Vec::new(),
			operations: None,
		}
	}

	#[must_use]
	pub fn operations(&self) -> Option<&[RawOperation]> {
		match self {
			Self::Entrypoint { operations } => Some(operations),
			Self::Module { operations, .. } => Some(operations.as_ref()?),
		}
	}

	pub fn operations_mut(&mut self) -> Option<&mut Vec<RawOperation>> {
		match self {
			Self::Entrypoint { operations } => Some(operations),
			Self::Module { operations, .. } => Some(operations.as_mut()?),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RawOperation {
	Nop,
}
