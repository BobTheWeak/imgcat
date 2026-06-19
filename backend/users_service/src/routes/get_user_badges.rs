use actix_web::web::{Data, Json};
use ic_actix::{AppStateRedis, AppStatePostgres};
use ic_actix::{ICError, ICResult};

use deadpool_postgres::tokio_postgres::types::Type;

use crate::db_objects::UserBadge;
use crate::redis_helpers;


pub async fn get_user_badges(
		//path: Path<ExpectedParams>,
		redis: Data<AppStateRedis>,
		//mariadb: Data<AppStateMariaDB>,
		postgres: Data<AppStatePostgres>,
		//request: HttpRequest,
		body: Json<Vec<i64>>,
	) -> ICResult<Json<Vec<UserBadge>>> {

	// Unwrap the body into a list
	let body = body.into_inner();

	// The first connection needed is Redis
	let mut r_conn = redis.get_conn()?;

	// TODO: Rate-limiter

	// Check Redis for cached badges, first
	let redis_result = redis_helpers::get_user_badges(&mut r_conn, &body)?;
	
	// We need to separate values into found & missing
	let mut found_values:Vec<UserBadge> = Vec::new();
	let mut missing_values:Vec<i64> = Vec::new();
	for (i, val) in redis_result.into_iter().enumerate() {
		match val {
			Some(v) => found_values.push(v),
			None => missing_values.push(body[i]),
		}
	}

	if missing_values.len() > 0 {

		let mut new_values:Vec<UserBadge> = {
			// Now, open connections to Postgres
			let conn = postgres.get_conn().await?;

			let Ok(result) = conn.query_typed(
				"SELECT id, username, link, badge FROM UsersSvc.GetUserBadges($1)", &[
				(&missing_values, Type::INT8_ARRAY)
			]).await else {return Err(ICError::POSTGRES_CONN)};

			result.iter().map(|x| UserBadge::from(x)).collect()
		};

		// Now, we need to add those values to the cache
		if new_values.len() > 0 {
			redis_helpers::set_user_badges(&mut r_conn, &new_values)?;
		}

		// Append the two lists
		found_values.append(&mut new_values);
	}

	return Ok(Json(found_values));
}