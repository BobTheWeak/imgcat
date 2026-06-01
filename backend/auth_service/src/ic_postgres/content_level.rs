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
