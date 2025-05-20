#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RawJsolFile {
	Entrypoint {
		operations: Vec<RawOperation>,
	},
	Module {
		#[serde(skip_serializing_if = "Vec::is_empty")]
		exports: Vec<()>,
		#[serde(skip_serializing_if = "Option::is_none")]
		operations: Option<Vec<RawOperation>>,
	},
}

impl RawJsolFile {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RawOperation {}
