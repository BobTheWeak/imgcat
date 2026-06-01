use serde_repr::{Serialize_repr, Deserialize_repr};
use tokio_postgres::types::{ToSql, FromSql};


#[derive(Debug, ToSql, FromSql, Serialize_repr, Deserialize_repr)]
#[postgres(name="VISIBILITY_LEVEL")]
#[repr(u8)]
pub enum VisibilityLevel {
	#[postgres(name="PRIVATE")]
	Private = 1,        // No visibility at all (except self)
	// Reserved for later
	//#[postgres(name="LIMITED")]
	//Limited = 2,        // Kids: visible to parents, groups
	//#[postgres(name="LIMITED FRIENDS")]
	//LimitedFriends = 3, // Kids: visible to parents, groups, friends
	#[postgres(name="FRIENDS")]
	Friends = 4,        // Adult: approval list
	#[postgres(name="PUBLIC")]
	Public = 5,         // Public visibility
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
//impl From<u8> for VisibilityLevel {
//	fn from(value:u8) -> Self {
//		match value {
//			1 => VisibilityLevel::Private,
//			2 => VisibilityLevel::FriendsOnly,
//			3 => VisibilityLevel::Public,
//			_ => VisibilityLevel::Private, // most restrictive
//		}
//	}
//}


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