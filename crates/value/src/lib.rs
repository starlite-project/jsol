#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Null,
	Bool(bool),
}

impl Serialize for Value {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self {
			Self::Null => serializer.serialize_unit(),
			Self::Bool(v) => serializer.serialize_bool(*v),
		}
	}
}

impl<'de> Deserialize<'de> for Value {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		todo!()
	}
}
