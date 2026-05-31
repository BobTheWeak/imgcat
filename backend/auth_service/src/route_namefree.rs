//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::get;
use actix_web::web::{Query, Data};
use actix_web::{HttpRequest, HttpResponse};

//use crate::libredis::AppStateRedis;
use crate::libpostgres::AppStatePostgres;
use crate::login_helpers::{validate_bearer_auth};

#[derive(Debug, Deserialize)]
struct UsernameParams {
	#[serde(rename="u")]
	username: String
}

#[get("/namefree")]
pub async fn namefree(
		params: Query<UsernameParams>,
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> HttpResponse {

	// TODO: Rate-limiter

	// Grab the Bearer header & check it's encoding
	let _jwt_string = match validate_bearer_auth(&request) {
		Ok(v) => v, Err(e) => return e.into()
	};

	let Ok(is_free) = postgres.is_username_free(&params.username).await else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Postgres")).finish();
	};

	return HttpResponse::Ok()
		.body(if is_free {"1"}else{"0"});
}
