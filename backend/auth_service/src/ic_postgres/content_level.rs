use tokio_postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name="CONTENT_LEVEL", rename_all="SCREAMING_SNAKE_CASE")]
pub enum ContentLevel {
	Prude,
	Dude,
	Lewd,
	Nude,
	Illegal,
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
impl From<u8> for ContentLevel {
	fn from(value:u8) -> Self {
		match value {
			1 => ContentLevel::Prude,
			2 => ContentLevel::Dude,
			3 => ContentLevel::Lewd,
			4 => ContentLevel::Nude,
			5 => ContentLevel::Illegal,
			_ => ContentLevel::Prude, // most restrictive
		}
	}
}

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