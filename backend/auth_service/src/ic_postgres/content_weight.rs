use serde_repr::{Serialize_repr, Deserialize_repr};
use tokio_postgres::types::{ToSql, FromSql};


#[derive(Debug, ToSql, FromSql, Serialize_repr, Deserialize_repr)]
#[postgres(name="content_weight")]
#[repr(u8)]
pub enum ContentWeight {
	#[postgres(name="NONE")]
	None = 0,

	#[postgres(name="MUCH LESS")]
	MuchLess = 2,
	#[postgres(name="LESS")]
	Less = 3,
	#[postgres(name="LITTLE LESS")]
	LittleLess = 4,
	#[postgres(name="NORMAL")]
	Normal = 5,
	#[postgres(name="LITTLE MORE")]
	LittleMore = 6,
	#[postgres(name="MORE")]
	More = 7,
	#[postgres(name="MUCH MORE")]
	MuchMore = 8,
}
