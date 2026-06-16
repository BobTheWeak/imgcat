use chrono::{Utc, TimeZone};
use serde::{Serialize, Deserialize};

use mysql::{Row as MariaRow, Value, FromRowError};
use mysql::prelude::{FromRow};


#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
	pub comment_id: i64,
	pub ts: i64,
	pub reply_to: Option<i64>,
	pub user_id: i64,
	pub img_link: Option<String>,
	pub text: String,
}

impl FromRow for Comment {
	fn from_row_opt(row:MariaRow) -> Result<Self, FromRowError> {
		Ok(Comment{
			comment_id: row.get("id").expect("Parsing Comment.comment_id"),
			ts: match row.get("ts").expect("Parsing Comment.ts") {
				Value::Date(yr,mo,dy,h,m,s,_) => {
					Utc.with_ymd_and_hms(
						yr as i32,
						mo as u32,
						dy as u32,
						h as u32,
						m as u32,
						s as u32
					).unwrap().timestamp()
				},
				_ => 0
			},
			user_id:  row.get("user_id").expect("Parsing Comment.user_id"),
			reply_to: row.get("reply_to").expect("Parsing Comment.reply_to"),
			img_link: row.get("img").expect("Parsing Comment.img_link"),
			text:     row.get("comment").expect("Parsing Comment.text"),
		})
	}
}