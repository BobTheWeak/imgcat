//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::get;
use actix_web::web::{Query, Data};
use actix_web::{HttpRequest, HttpResponse};

//use crate::libredis::AppStateRedis;
use crate::libjwt::validate;
use crate::ic_postgres::AppStatePostgres;
use crate::login_helpers::{get_bearer_auth};

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
	let jwt_string = match get_bearer_auth(&request) {
		Ok(v) => v, Err(e) => return e.into()
	};

	// Verify it (we don't care which token is sent)
	if !validate(jwt_string) {
		return HttpResponse::Forbidden()
			.insert_header(("IC-Error","Header validation")).finish();
	}

	let Ok(is_free) = postgres.is_username_free(&params.username).await else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Postgres")).finish();
	};

	return HttpResponse::Ok()
		.body(if is_free {"1"}else{"0"});
}
