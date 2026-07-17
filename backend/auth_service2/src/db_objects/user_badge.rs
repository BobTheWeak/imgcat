use serde::{Serialize, Deserialize};

use deadpool_postgres::tokio_postgres::{Row as PostgresRow};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBadge {
	pub id: i64,
	pub username: String,
	pub link: String,
	pub badge: char, // NOTE: I'd like to type this with the Enum below, but I couldn't get Serde to just serialize/deserialize as a char.
}

impl From<&PostgresRow> for UserBadge {
	fn from(row:&PostgresRow) -> Self {
		//let badge_char:char = row.get::<_,&str>("badge").chars().nth(0).unwrap();
		//Self {
		//	id:        row.get::<_,i64>("id"),
		//	username:  row.get::<_,String>("username"),
		//	link:      row.get::<_,String>("link"),
		//	badge:     BadgeType::from(badge_char),
		//}
		Self {
			id:        row.get::<_,i64>("id"),
			username:  row.get::<_,String>("username"),
			link:      row.get::<_,String>("link"),
			badge:     row.get::<_,&str>("badge").chars().nth(0).unwrap(),
		}
	}
}

//#[derive(Debug, Serialize, Deserialize)]
//pub enum BadgeType {
//	Staff = 'S',
//	Moderator = 'M',
//	OfficialOrg = 'O',
//	VerifiedUser = 'V',
//	User = 'U',
//}
//
//impl From<char> for BadgeType {
//	fn from(val:char) -> Self {
//		match val {
//			'S' => Self::Staff,
//			'M' => Self::Moderator,
//			'O' => Self::OfficialOrg,
//			'V' => Self::VerifiedUser,
//			'U' | _ => Self::User
//		}
//	}
//}