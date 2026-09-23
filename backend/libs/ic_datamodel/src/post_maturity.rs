use serde::{Serialize, Deserialize};

#[cfg(feature = "postgres")]
use tokio_postgres::{Row as PostgresRow};

use crate::maturity::Maturity;
use crate::serde_helpers::{bool_as_01};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMaturity {
	// We want to compress the datastream as much as possible
	// TODO: Transform bool into 0/1
	#[serde(rename="id")]
	pub post_id:   i64,

	#[serde(rename="m")]
	pub maturity:  Maturity,

	#[serde(rename="s", with="bool_as_01")]
	pub is_sexual: bool,

	#[serde(rename="g", with="bool_as_01")]
	pub is_gore:   bool,

	#[serde(rename="t", with="bool_as_01")]
	pub is_trauma: bool,
}


#[cfg(feature = "postgres")]
impl From<&PostgresRow> for PostMaturity {
	fn from(row:&PostgresRow) -> Self {
		Self {
			post_id:   row.get::<_,i64>("post_id"),
			maturity:  row.get::<_,Maturity>("maturity"),
			is_sexual: row.get::<_,bool>("is_sexual"),
			is_gore:   row.get::<_,bool>("is_gore"),
			is_trauma: row.get::<_,bool>("is_trauma"),
		}
	}
}
