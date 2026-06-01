use actix_web::get;
use actix_web::web::{Data};
use actix_web::{HttpRequest, HttpResponse};

use crate::libredis::AppStateRedis;
use crate::ic_postgres::AppStatePostgres;
use crate::libjwt::{AuthJwt, RefreshJwt, DecodeJwt};
use crate::login_helpers::{validate_bearer_auth, get_auth_jwt, get_refresh_cookie, get_auth_cookie};

#[get("/refresh")]
pub async fn refresh(
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> HttpResponse {

	// Grab the Bearer header & check it's encoding
	let jwt_string = match validate_bearer_auth(&request) {
		Ok(v) => v, Err(e) => return e.into()
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
		.cookie(get_refresh_cookie(&rjwt))
		.cookie(get_auth_cookie(&ajwt))
		.finish();
}
