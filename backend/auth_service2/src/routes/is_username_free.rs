use serde::Deserialize;
use actix_web::web::{Query, Path, Data};
use actix_web::{HttpRequest};
use regex::regex;

//use crate::libredis::AppStateRedis;
use ic_jwt::validate_with_defaults;
use ic_actix::{ICResult, ICError, AppStatePostgres, get_bearer_jwt, };
use crate::postgres_helpers::{is_username_free as pg_namefree};

fn validate_username(username:&str) -> ICResult<()> {
	// Support alphanumeric + ".-_~"
	// TODO: Support unicode accents + more punctuation
	let regex = regex!(r"^[0-9A-Za-z \.\-_~]{4,40}$");
	if username.len() < 4 {
		return Err(ICError::error("Username is too short"));
	}
	if username.len() > 40 {
		return Err(ICError::error("Username is too long"));
	}
	if !regex.is_match(username) {
		return Err(ICError::error("Unsupported symbols (.-_~ only)"));
	}
	return Ok(());
}


#[derive(Debug, Deserialize)]
pub struct QueryParams {
	#[serde(rename="u")]
	username: String
}

// Expects: /nf?u=BobNameHere
pub async fn is_username_free_by_query(
		params: Query<QueryParams>,
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<&'static str> {

	// TODO: Rate-limiter

	validate_username(&params.username)?;

	// Grab the Bearer token & verify its ours
	if validate_with_defaults(get_bearer_jwt(&request)?) {
		// Grab the DB and check
		let p_conn = postgres.get_conn().await?;
		if pg_namefree(&p_conn, &params.username).await? {
			Ok("1")
		} else {
			Ok("0")
		}
	} else {
		Ok("0")
	}
}



#[derive(Debug, Deserialize)]
pub struct PathParams {
	username: String
}

// Expects: /nf/{username}
pub async fn is_username_free_by_path(
		params: Path<PathParams>,
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<&'static str> {

	// TODO: Rate-limiter

	validate_username(&params.username)?;

	// Grab the Bearer token & verify its ours
	if validate_with_defaults(get_bearer_jwt(&request)?) {
		// Grab the DB and check
		let p_conn = postgres.get_conn().await?;
		if pg_namefree(&p_conn, &params.username).await? {
			Ok("1")
		} else {
			Ok("0")
		}
	} else {
		Ok("0")
	}
}