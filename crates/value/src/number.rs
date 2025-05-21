use std::{
	fmt::{Debug, Display, Formatter, Result as FmtResult, Write as _},
	hash::{Hash, Hasher},
};

use serde::{
	Deserialize, Deserializer, Serialize, Serializer,
	de::{Error as DeError, Visitor},
};

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Number(N);

impl Number {
	#[must_use]
	pub const fn is_i64(&self) -> bool {
		match self.0 {
			N::PosInt(v) => v <= i64::MAX as u64,
			N::NegInt(_) => true,
			N::Float(_) => false,
		}
	}

	#[must_use]
	pub const fn is_u64(&self) -> bool {
		matches!(self.0, N::PosInt(_))
	}

	#[must_use]
	pub const fn is_f64(&self) -> bool {
		matches!(self.0, N::Float(_))
	}

	#[must_use]
	pub const fn as_i64(&self) -> Option<i64> {
		match self.0 {
			N::PosInt(n) => {
				if n <= i64::MAX as u64 {
					Some(n as i64)
				} else {
					None
				}
			}
			N::NegInt(i) => Some(i),
			N::Float(_) => None,
		}
	}

	#[must_use]
	pub const fn as_u64(&self) -> Option<u64> {
		let N::PosInt(n) = self.0 else {
			return None;
		};

		Some(n)
	}

	#[must_use]
	pub const fn as_f64(&self) -> Option<f64> {
		match self.0 {
			N::PosInt(n) => Some(n as f64),
			N::NegInt(n) => Some(n as f64),
			N::Float(n) => Some(n),
		}
	}

	#[must_use]
	pub const fn from_f64(f: f64) -> Option<Self> {
		if f.is_finite() {
			let n = N::Float(f);

			Some(Self(n))
		} else {
			None
		}
	}

	#[must_use]
	pub const fn as_i128(&self) -> Option<i128> {
		match self.0 {
			N::PosInt(n) => Some(n as i128),
			N::NegInt(n) => Some(n as i128),
			N::Float(_) => None,
		}
	}

	#[must_use]
	pub const fn as_u128(&self) -> Option<u128> {
		let N::PosInt(n) = self.0 else {
			return None;
		};

		Some(n as u128)
	}

	#[must_use]
	pub fn from_i128(i: i128) -> Option<Self> {
		let n = {
			if let Ok(n) = u64::try_from(i) {
				N::PosInt(n)
			} else if let Ok(i) = i64::try_from(i) {
				N::NegInt(i)
			} else {
				return None;
			}
		};

		Some(Self(n))
	}

	#[must_use]
	pub fn from_u128(i: u128) -> Option<Self> {
		let n = {
			if let Ok(u) = u64::try_from(i) {
				N::PosInt(u)
			} else {
				return None;
			}
		};

		Some(Self(n))
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("Number(")?;
		Display::fmt(&self, f)?;
		f.write_char(')')
	}
}

impl<'de> Deserialize<'de> for Number {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(NumberVisitor)
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self.0 {
			N::PosInt(u) => f.write_str(itoa::Buffer::new().format(u)),
			N::NegInt(i) => f.write_str(itoa::Buffer::new().format(i)),
			N::Float(fl) => f.write_str(ryu::Buffer::new().format_finite(fl)),
		}
	}
}

impl Serialize for Number {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self.0 {
			N::PosInt(u) => serializer.serialize_u64(u),
			N::NegInt(i) => serializer.serialize_i64(i),
			N::Float(f) => serializer.serialize_f64(f),
		}
	}
}

macro_rules! impl_from_unsigned {
    ($($ty:ty),*) => {
        $(
            #[allow(clippy::cast_lossless)]
            impl ::std::convert::From<$ty> for $crate::Number {
                fn from(i: $ty) -> Self {
                    let n = $crate::number::N::PosInt(i as u64);

                    Self(n)
                }
            }
        )*
    };
}

macro_rules! impl_from_signed {
    ($($ty:ty),*) => {
        $(
            #[allow(clippy::cast_lossless)]
            impl ::std::convert::From<$ty> for $crate::Number {
                fn from(i: $ty) -> Self {
                    let n = {
                        if i < 0 {
                            $crate::number::N::NegInt(i as i64)
                        } else {
                            $crate::number::N::PosInt(i as u64)
                        }
                    };

                    Self(n)
                }
            }
        )*
    };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);

impl_from_signed!(i8, i16, i32, i64, isize);

struct NumberVisitor;

impl Visitor<'_> for NumberVisitor {
	type Value = Number;

	fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
		formatter.write_str("a JSON number")
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(v.into())
	}

	fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Number::from_i128(v).ok_or_else(|| DeError::custom("JSON number out of range"))
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(v.into())
	}

	fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Number::from_u128(v).ok_or_else(|| DeError::custom("JSON number out of range"))
	}

	fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Number::from_f64(v).ok_or_else(|| DeError::custom("not a JSON number"))
	}
}

#[derive(Clone, Copy)]
enum N {
	PosInt(u64),
	NegInt(i64),
	Float(f64),
}

impl Eq for N {}

impl Hash for N {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match *self {
			Self::PosInt(i) => i.hash(state),
			Self::NegInt(i) => i.hash(state),
			Self::Float(f) => {
				if f == 0.0f64 {
					0.0f64.to_bits().hash(state);
				} else {
					f.to_bits().hash(state);
				}
			}
		}
	}
}

impl PartialEq for N {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::PosInt(a), Self::PosInt(b)) => a == b,
			(Self::NegInt(a), Self::NegInt(b)) => a == b,
			(Self::Float(a), Self::Float(b)) => a == b,
			_ => false,
		}
	}
}
