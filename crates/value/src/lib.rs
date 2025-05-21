#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod number;
mod ty;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write as _};

use serde::{
	Deserialize, Deserializer, Serialize, Serializer,
	de::{
		Error as DeError, SeqAccess, Visitor,
		value::{I128Deserializer, U128Deserializer},
	},
};

pub use self::{number::Number, ty::Type};

#[derive(Clone, PartialEq, Eq)]
pub enum Value {
	Null,
	Bool(bool),
	Number(Number),
	String(String),
	Array(Vec<Self>),
}

impl Value {
	#[must_use]
	pub const fn r#type(&self) -> Type {
		match self {
			Self::Null => Type::Null,
			Self::Bool(_) => Type::Bool,
			Self::Array(_) => Type::Array,
			Self::String(_) => Type::String,
			Self::Number(n) if n.is_f64() => Type::Float,
			Self::Number(_) => Type::Int,
		}
	}
}

impl Debug for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Null => f.write_str("Null")?,
			Self::Bool(boolean) => {
				f.write_str("Bool(")?;
				Display::fmt(&boolean, f)?;
				f.write_char(')')?;
			}
			Self::Number(n) => Debug::fmt(n, f)?,
			Self::String(s) => {
				f.write_str("String(")?;
				f.write_str(s)?;
				f.write_char(')')?;
			}
			Self::Array(vec) => {
				f.write_str("Array ")?;
				Debug::fmt(&vec, f)?;
			}
		}

		Ok(())
	}
}

impl<'de> Deserialize<'de> for Value {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(ValueVisitor)
	}
}

impl Serialize for Value {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self {
			Self::Null => serializer.serialize_unit(),
			Self::Bool(v) => serializer.serialize_bool(*v),
			Self::Number(n) => n.serialize(serializer),
			Self::String(s) => serializer.serialize_str(s),
			Self::Array(v) => v.serialize(serializer),
		}
	}
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
	type Value = Value;

	fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
		formatter.write_str("any valid JSON value")
	}

	fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
		Ok(Value::Bool(v))
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(Value::Number(v.into()))
	}

	fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		let de = I128Deserializer::new(v);
		Number::deserialize(de).map(Value::Number)
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
		Ok(Value::Number(v.into()))
	}

	fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		let de = U128Deserializer::new(v);
		Number::deserialize(de).map(Value::Number)
	}

	fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
		Ok(Number::from_f64(v).map_or(Value::Null, Value::Number))
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		self.visit_string(v.to_owned())
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
		Ok(Value::String(v))
	}

	fn visit_none<E>(self) -> Result<Self::Value, E> {
		Ok(Value::Null)
	}

	fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		Deserialize::deserialize(deserializer)
	}

	fn visit_unit<E>(self) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(Value::Null)
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let mut vec = if let Some(size) = seq.size_hint() {
			Vec::with_capacity(size)
		} else {
			Vec::new()
		};

		while let Some(elem) = seq.next_element()? {
			vec.push(elem);
		}

		Ok(Value::Array(vec))
	}
}
