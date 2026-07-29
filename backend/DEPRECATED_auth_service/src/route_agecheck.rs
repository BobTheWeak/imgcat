/*

	NOTE: This is not used, at the moment
	But if we do need it, this works as expected

//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::get;
use actix_web::web::{Query, Data};
use actix_web::{HttpResponse};

//use crate::libredis::AppStateRedis;
use crate::libpostgres::AppStatePostgres;

#[derive(Debug, Deserialize)]
struct AgeCheckParams {
	#[serde(rename="cc")]
	country_code: String,
	#[serde(rename="sc")]
	state_code: Option<String>,
}

#[get("/agecheck")]
pub async fn agecheck(
		params: Query<AgeCheckParams>,
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		//request: HttpRequest,
	) -> HttpResponse {

	// TODO: Rate-limiter


	let Ok(age_check) = postgres.is_age_needed_on_signup(&params.country_code, params.state_code.as_deref()).await else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Postgres")).finish();
	};

	return HttpResponse::Ok()
		.body(if age_check {"1"}else{"0"});
}
*/