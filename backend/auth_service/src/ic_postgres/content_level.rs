use serde_repr::{Serialize_repr, Deserialize_repr};
use tokio_postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql, Serialize_repr, Deserialize_repr)]
#[postgres(name="CONTENT_LEVEL")]
#[repr(u8)]
pub enum ContentLevel {
	#[postgres(name="PRUDE")]
	Prude = 1,
	#[postgres(name="DUDE")]
	Dude = 2,
	#[postgres(name="LEWD")]
	Lewd = 3,
	#[postgres(name="NUDE")]
	Nude = 4,
	#[postgres(name="ILLEGAL")]
	Illegal = 5,
}

//impl From<&str> for ContentLevel {
//	fn from(value:&str) -> Self {
//		match value.to_ascii_uppercase().as_ref() {
//			"PRUDE" => ContentLevel::Prude,
//			"DUDE" => ContentLevel::Dude,
//			"LEWD" => ContentLevel::Lewd,
//			"NUDE" => ContentLevel::Nude,
//			"ILLEGAL" => ContentLevel::Illegal,
//			_ => ContentLevel::Prude, // most restrictive
//		}
//	}
//}

//impl From<String> for ContentLevel {
//	fn from(value:String) -> Self {
//		match value.to_ascii_uppercase().as_ref() {
//			"PRUDE" => ContentLevel::Prude,
//			"DUDE" => ContentLevel::Dude,
//			"LEWD" => ContentLevel::Lewd,
//			"NUDE" => ContentLevel::Nude,
//			"ILLEGAL" => ContentLevel::Illegal,
//			_ => ContentLevel::Prude, // most restrictive
//		}
//	}
//}

// For expected binary-storage translation
//impl From<u8> for ContentLevel {
//	fn from(value:u8) -> Self {
//		match value {
//			1 => ContentLevel::Prude,
//			2 => ContentLevel::Dude,
//			3 => ContentLevel::Lewd,
//			4 => ContentLevel::Nude,
//			5 => ContentLevel::Illegal,
//			_ => ContentLevel::Prude, // most restrictive
//		}
//	}
//}

// For expected PostgreSQL translation
//impl From<i16> for ContentLevel {
//	fn from(value:i16) -> Self {
//		match value {
//			1 => ContentLevel::Prude,
//			2 => ContentLevel::Dude,
//			3 => ContentLevel::Lewd,
//			4 => ContentLevel::Nude,
//			5 => ContentLevel::Illegal,
//			_ => ContentLevel::Prude, // most restrictive
//		}
//	}
//}