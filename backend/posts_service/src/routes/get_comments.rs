use std::collections::HashMap;

use serde::{Serialize, Deserialize};
//use actix_web::{HttpRequest};
use actix_web::web::{Path, Data, Json};
//use ic_jwt::{AuthJwt, DecodeJwt};
use ic_actix::{AppStateMariaDB};
use ic_actix::{ICError, ICResult};
//use ic_actix::{get_bearer_jwt};

use mysql::prelude::Queryable;

use crate::db_objects::comment::Comment;
//use crate::redis_helpers;


#[derive(Serialize, Deserialize)]
pub struct ExpectedParams {
	// TODO: Tried to do [char;12], but a length-check needs to be a custom Guard
	post_link:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedReturn {
	// This is a flat array of each comment, indexed by comment_id
	pub comments: HashMap<i64, Comment>,
	// This is a flattened, reverse index of the structure of replies
	pub replies: HashMap<i64, Vec<i64>>,
}

pub async fn get_comments(
		path: Path<ExpectedParams>,
		//redis: Data<AppStateRedis>,
		mariadb: Data<AppStateMariaDB>,
		//request: HttpRequest,
	) -> ICResult<Json<ExpectedReturn>> {
	
	// This is basic validation, making sure the post_link is our expected format
	// TODO: Make a custom guard & let actix handle this
	if path.post_link.len()!=12 {return Err(ICError::LINK_NOT_FOUND)}

	// The first connection needed is Redis
	//let mut r_conn = redis.get_conn()?;

	// TODO: Rate-limiter

	// Bearer header is OPTIONAL
	//if let Ok(jwt_string) = get_bearer_jwt(&request) {
	//	let Some(ajwt) = AuthJwt::decode_with_defaults(jwt_string).ok() else {
	//		return Err(ICError::HEADER_VALIDATION);
	//	};
	//	// Do we want to do this?
	//	check_temp_ban(ajwt.sub, &mut r_conn).await?;
	//};

	// Now, open connections to MariaDB
	let mut m_conn = mariadb.get_conn()?;
	
	// Posts are in MariaDB
	let Ok(mut comments):Result<Vec<Comment>, _> = m_conn.exec(
		"CALL Comments.GetPostComments(Posts.GetPostIdByLink(?))",
		(&path.post_link,)
	) else {return Err(ICError::MARIADB_CONN)};

	// For now, sort all comments by date
	// TODO: Sort by comment upvotes (this is NOT supported yet)
	comments.sort_by(|a,b| {a.ts.cmp(&b.ts)} );

	// We're not doing this here... Badges should be a separate API call, because
	// those are easily cachable & static, but comments can change more frequently.
	// // Build a list of all the unique users who have commented
	// let mut user_ids: HashSet<i64> = comments.iter().map(|x|{x.user_id}).collect();
	// let user_ids:Vec<i64> = user_ids.into_iter().collect(); // Redef as Vector
	// 
	// // Pull a list of User badges
	// let users:Vec<User> = {
	// 	let conn = postgres.get_conn().await?;
	// 	let Ok(result) = conn.query_typed(
	// 		"SELECT id, username, link, badge FROM UserDB.GetUsersBadges($1)",
	// 		&[(&user_ids, Type::INT8_ARRAY)]
	// 	).await else {return Err(ICError::POSTGRES_CONN)};
	// 
	// 	result.iter().map(|x| User::from(x)).collect()
	// };

	// Iterate through comments again, and build an index of replies
	let mut replies:HashMap<i64, Vec<i64>>  = HashMap::new();
	for c in comments.iter() {
		// If it's a root comment, just use 0
		let parent:i64 = c.reply_to.unwrap_or(0);
		
		if let Some(v) = replies.get_mut(&parent) {
			v.push(c.comment_id);
		} else {
			let mut v:Vec<i64> = Vec::new();
			v.push(c.comment_id);
			replies.insert(parent, v);
		}
	}

	return Ok(Json(ExpectedReturn {
		comments: comments.into_iter().map(|x| (x.comment_id, x)).collect(),
		replies,
	}));
}