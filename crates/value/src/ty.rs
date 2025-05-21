use std::fmt::{Formatter, Result as FmtResult};

use serde::{
	Deserialize, Deserializer, Serialize, Serializer,
	de::{EnumAccess, Error as DeError, Unexpected, VariantAccess, Visitor},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
	Any,
	Null,
	Bool,
	Int,
	Float,
	String,
	Array,
}

impl<'de> Deserialize<'de> for Type {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_enum("Type", VARIANTS, TypeVisitor)
	}
}

impl Serialize for Type {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match *self {
			Self::Any => serializer.serialize_unit_variant("Type", 0, "any"),
			Self::Null => serializer.serialize_unit_variant("Type", 1, "null"),
			Self::Bool => serializer.serialize_unit_variant("Type", 2, "bool"),
			Self::Int => serializer.serialize_unit_variant("Type", 3, "int"),
			Self::Float => serializer.serialize_unit_variant("Type", 4, "float"),
			Self::String => serializer.serialize_unit_variant("Type", 5, "string"),
			Self::Array => serializer.serialize_unit_variant("Type", 6, "array"),
		}
	}
}

const VARIANTS: &[&str] = &[
	"any", "null", "bool", "boolean", "int", "number", "float", "double", "string", "array",
];

struct TypeVisitor;

impl<'de> Visitor<'de> for TypeVisitor {
	type Value = Type;

	fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
		formatter.write_str("enum Type")
	}

	fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
	where
		A: EnumAccess<'de>,
	{
		let (ty, v) = data.variant()?;

		v.unit_variant()?;
		match ty {
			TypeField::Any => Ok(Type::Any),
			TypeField::Null => Ok(Type::Null),
			TypeField::Bool => Ok(Type::Bool),
			TypeField::Int => Ok(Type::Int),
			TypeField::Float => Ok(Type::Float),
			TypeField::String => Ok(Type::String),
			TypeField::Array => Ok(Type::Array),
		}
	}
}

enum TypeField {
	Any,
	Null,
	Bool,
	Int,
	Float,
	String,
	Array,
}

impl<'de> Deserialize<'de> for TypeField {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(TypeFieldVisitor)
	}
}

struct TypeFieldVisitor;

impl Visitor<'_> for TypeFieldVisitor {
	type Value = TypeField;

	fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
		formatter.write_str("variant identifier")
	}

	fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
		match v {
			0 => Ok(TypeField::Any),
			1 => Ok(TypeField::Null),
			2 => Ok(TypeField::Bool),
			3 => Ok(TypeField::Int),
			4 => Ok(TypeField::Float),
			5 => Ok(TypeField::String),
			6 => Ok(TypeField::Array),
			other => Err(DeError::invalid_value(
				Unexpected::Unsigned(other),
				&"variant index 0 <= i < 7",
			)),
		}
	}

	fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
		match v {
			"any" => Ok(TypeField::Any),
			"null" => Ok(TypeField::Null),
			"bool" | "boolean" => Ok(TypeField::Bool),
			"int" | "number" => Ok(TypeField::Int),
			"float" | "double" => Ok(TypeField::Float),
			"string" => Ok(TypeField::String),
			"array" => Ok(TypeField::Array),
			other => Err(DeError::unknown_variant(other, VARIANTS)),
		}
	}

	fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Self::Value, E> {
		match v {
			b"any" => Ok(TypeField::Any),
			b"null" => Ok(TypeField::Null),
			b"bool" | b"boolean" => Ok(TypeField::Bool),
			b"int" | b"number" => Ok(TypeField::Int),
			b"float" | b"double" => Ok(TypeField::Float),
			b"string" => Ok(TypeField::String),
			b"array" => Ok(TypeField::Array),
			other => {
				let other = String::from_utf8_lossy(other);
				Err(DeError::unknown_variant(&other, VARIANTS))
			}
		}
	}
}
