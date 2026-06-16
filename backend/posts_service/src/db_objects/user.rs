use serde::{Serialize, Deserialize};

use deadpool_postgres::tokio_postgres::{Row as PostgresRow};


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserBadge {
	pub id: i64,
	pub link: String,
	pub username: String,
	pub r#type: UserType,
}

impl From<&PostgresRow> for UserBadge {
	fn from(row:&PostgresRow) -> Self {
		// TODO: Hacky... Needs to be an enum in the DB
		let badge_char:String = row.get::<_,String>("badge");
		Self {
			id:     row.get::<_,i64>("id"),
			link:   row.get::<_,String>("link"),
			username:    row.get::<_,String>("username"),
			r#type: match badge_char.chars().nth(0).unwrap() {
				'U' => UserType::User,
				'O' => UserType::OriginalPoster,
				'M' => UserType::Moderator,
				'S' => UserType::Staff,
				_ => UserType::Deleted,
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserType {
	Deleted = 0,
	User = 1,
	OriginalPoster = 2,
	Moderator = 3,
	Staff = 4,
}
