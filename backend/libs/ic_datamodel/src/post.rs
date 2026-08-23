use serde::{Serialize, Deserialize};

#[cfg(feature="mariadb")]
use mysql_common::row::{Row, convert::FromRow, convert::FromRowError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
	pub post_id: i64,
	pub ts: i64,
	pub title: Option<String>,
	pub user_id: i64,
	pub is_public: bool,
	pub link: String,
}

#[cfg(feature="mariadb")]
impl FromRow for Post {
	fn from_row_opt(row:Row) -> Result<Self, FromRowError> {
		Ok(Post{
			post_id:   row.get("id").expect("Parsing Post.post_id"),
			ts:        row.get("time").expect("Parsing Post.time"),
			title:     row.get("title").expect("Parsing Post.title"),
			user_id:   row.get("user_id").expect("Parsing Post.user_id"),
			is_public: row.get("is_public").expect("Parsing Post.is_public"),
			link:      row.get("link").expect("Parsing Post.link"),
		})
	}
}