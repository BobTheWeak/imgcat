use core::error::Error;
//use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[cfg(feature = "postgres")]
use postgres_types::{Type, ToSql, FromSql, accepts};
#[cfg(feature = "postgres")]
use postgres_protocol::types::{int2_from_sql};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "postgres", derive(ToSql))]
#[repr(u8)]
pub enum Maturity {
	Prude = 1,
	Dude = 2,
	Lewd = 3,
	Nude = 4,
	Illegal = 5,
}

impl TryFrom<u8> for Maturity {
	type Error = &'static str;
	fn try_from(v:u8) -> Result<Self, &'static str> {
		match v {
			1 => Ok(Maturity::Prude),
			2 => Ok(Maturity::Dude),
			3 => Ok(Maturity::Lewd),
			4 => Ok(Maturity::Nude),
			5 => Ok(Maturity::Illegal),
			_ => Err("invalid maturity")
		}
	}
}

impl TryFrom<i16> for Maturity {
	type Error = &'static str;
	fn try_from(v:i16) -> Result<Self, &'static str> {
		match v {
			1 => Ok(Maturity::Prude),
			2 => Ok(Maturity::Dude),
			3 => Ok(Maturity::Lewd),
			4 => Ok(Maturity::Nude),
			5 => Ok(Maturity::Illegal),
			_ => Err("invalid maturity")
		}
	}
}


impl<'a> FromSql<'a> for Maturity {
	fn from_sql(_:&Type, raw:&'a [u8]) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
		int2_from_sql(raw)?.try_into().or(Err("parse error: maturity".into()))
	}

	accepts!(INT2);
}