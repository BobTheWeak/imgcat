use serde::{Serialize, Deserialize};

use crate::media_type::MediaType;

#[cfg(feature="mariadb")]
use mysql_common::row::{Row, convert::FromRow, convert::FromRowError};


#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
	pub media_id: i64,
	pub media_type: MediaType,
	pub mime_type: Option<String>,
	pub link: String,
}

#[cfg(feature="mariadb")]
impl FromRow for Media {
	fn from_row_opt(row:Row) -> Result<Self, FromRowError> {
		Ok(Self {
			media_id:   row.get("id").expect("Parsing Media.media_id"),
			media_type: row.get("media_type").expect("Parsing Media.media_type"),
			mime_type:  row.get("mime_type").expect("Parsing Media.mime_type"),
			link:       row.get("link").expect("Parsing Media.link"),
		})
	}
}
