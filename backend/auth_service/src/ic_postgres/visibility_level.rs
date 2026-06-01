use tokio_postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name="VISIBILITY_LEVEL", rename_all="SCREAMING_SNAKE_CASE")]
pub enum VisibilityLevel {
	Private,
	FriendsOnly,
	Public,
}

//impl From<&str> for VisibilityLevel {
//	fn from(value:&str) -> Self {
//		match value.to_ascii_uppercase().as_ref() {
//			"PRIVATE" => VisibilityLevel::Private,
//			"FRIENDS ONLY" => VisibilityLevel::FriendsOnly,
//			"PUBLIC" => VisibilityLevel::Public,
//			_ => VisibilityLevel::Private, // most restrictive
//		}
//	}
//}

//impl From<String> for VisibilityLevel {
//	fn from(value:String) -> Self {
//		match value.to_ascii_uppercase().as_ref() {
//			"PRIVATE" => VisibilityLevel::Private,
//			"FRIENDS ONLY" => VisibilityLevel::FriendsOnly,
//			"PUBLIC" => VisibilityLevel::Public,
//			_ => VisibilityLevel::Private, // most restrictive
//		}
//	}
//}

// For expected binary-storage translation
impl From<u8> for VisibilityLevel {
	fn from(value:u8) -> Self {
		match value {
			1 => VisibilityLevel::Private,
			2 => VisibilityLevel::FriendsOnly,
			3 => VisibilityLevel::Public,
			_ => VisibilityLevel::Private, // most restrictive
		}
	}
}

//// For expected PostgreSQL translation
//impl From<i16> for VisibilityLevel {
//	fn from(value:i16) -> Self {
//		match value {
//			1 => VisibilityLevel::Private,
//			2 => VisibilityLevel::FriendsOnly,
//			3 => VisibilityLevel::Public,
//			_ => VisibilityLevel::Private, // most restrictive
//		}
//	}
//}