use serde::{Serialize, Deserialize};
//use actix_web::{HttpRequest};
use actix_web::web::{Path, Data, Json};
use ic_actix::{AppStateRedis, AppStateMariaDB};
use ic_actix::{ICError, ICResult};

use mysql::prelude::Queryable;

use crate::redis_helpers::{get_post_views, set_post_views};


#[derive(Serialize, Deserialize)]
pub struct ExpectedParams {
	// TODO: Tried to do [char;12], but a length-check needs to be a custom Guard
	post_link:String
}


pub async fn get_views(
		path: Path<ExpectedParams>,
		redis: Data<AppStateRedis>,
		mariadb: Data<AppStateMariaDB>,
		//request: HttpRequest,
	) -> ICResult<Json<u32>> {
	
	// This is basic validation, making sure the post_link is our expected format
	// TODO: Make a custom guard & let actix handle this
	if path.post_link.len()!=12 {return Err(ICError::LINK_NOT_FOUND)}

	// The first connection needed is Redis
	let mut r_conn = redis.get_conn()?;

	// TODO: Rate-limiter

	// Bearer header is OPTIONAL
	//if let Ok(jwt_string) = get_bearer_jwt(&request) {
	//	let Some(ajwt) = AuthJwt::decode_with_defaults(jwt_string).ok() else {
	//		return Err(ICError::HEADER_VALIDATION);
	//	};
	//	// Do we want to do this?
	//	check_temp_ban(ajwt.sub, &mut r_conn).await?;
	//};

	// Check Redis, and maybe that's all we need
	if let Some(views) = get_post_views(&mut r_conn, &path.post_link)? {
		return Ok(Json(views));
	}

	// Darn... We have to go to the DB
	let mut m_conn = mariadb.get_conn()?;

	// Posts are in MariaDB
	let Ok(views) = m_conn.exec_first(
		"SELECT Posts.GetViews(Posts.GetPostIdByLink(?))",
		(&path.post_link,)
	) else {return Err(ICError::MARIADB_CONN)};

	let views = views.unwrap();

	// If we get an error, we don't care
	// TODO: Log this, because it probably means something
	let _ = set_post_views(&mut r_conn, &path.post_link, views);

	return Ok(Json(views));
}