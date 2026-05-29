//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::get;
use actix_web::web::{Query, Data};
use actix_web::{HttpRequest, HttpResponse};

//use crate::libredis::AppStateRedis;
use crate::libpostgres::AppStatePostgres;
use crate::libjwt::{validate};

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
	let Some(jwt_string) = request.headers().get("Authorization") else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header")).finish();
	};
	let Ok(jwt_string) = jwt_string.to_str() else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header")).finish();
	};
	let Some(jwt_string) = jwt_string.strip_prefix("Bearer ") else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header")).finish();
	};

	// Validate the JWT & make sure it's ours
	// We don't care what JWT was passed in, or need to view the data itself
	if !validate(jwt_string) {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header validation")).finish();
	};

	let Ok(is_free) = postgres.is_username_free(&params.username).await else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Postgres")).finish();
	};

	return HttpResponse::Ok()
		.body(if is_free {"1"}else{"0"});
}
