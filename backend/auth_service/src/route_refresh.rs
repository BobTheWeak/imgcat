use actix_web::get;
use actix_web::web::{Data};
use actix_web::{HttpRequest, HttpResponse};

use crate::libredis::AppStateRedis;
use crate::libpostgres::AppStatePostgres;
use crate::libjwt::{AuthJwt, RefreshJwt, DecodeJwt};
use crate::login_helpers::{get_auth_jwt, get_refresh_cookie, get_auth_cookie};

#[get("/refresh")]
pub async fn refresh(
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> HttpResponse {

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

	// Decode the JWT & make sure it's ours
	let Ok(rjwt) = RefreshJwt::decode(jwt_string) else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header validation")).finish();
	};

	// Generate a new Auth JWT
	let ajwt:AuthJwt = match get_auth_jwt(&rjwt, &redis, &postgres).await {
		Ok(v) => v, Err(e) => return e.into()
	};

	// Return both JWTs and the data of the auth JWT in the body
	return HttpResponse::Ok()
		.content_type("application/json")
		.cookie(get_refresh_cookie(&rjwt))
		.cookie(get_auth_cookie(&ajwt))
		.finish();
}
