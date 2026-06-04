use serde_repr::{Serialize_repr, Deserialize_repr};
use tokio_postgres::types::{ToSql, FromSql};


#[derive(Debug, ToSql, FromSql, Serialize_repr, Deserialize_repr)]
#[postgres(name="visibility_level")]
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
	Public = 5,         // Public, to other ImgCat users
	
	#[postgres(name="GLOBAL")]
	Global = 9,         // Public, to anyone surfing the interwebs
}

// This is to work around some of the limitations of SQL Type translations
impl VisibilityLevel {
	pub const fn to_sql_text(&self) -> &'static str {
		match self {
			Self::Private => "PRIVATE",
			Self::Friends => "FRIENDS",
			Self::Public => "PUBLIC",
			Self::Global => "GLOBAL",
		}
	}
}
