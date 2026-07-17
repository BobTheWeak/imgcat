use actix_web::web::{Data};
use actix_web::{HttpRequest, HttpResponse};

use ic_jwt::{AuthJwt, RefreshJwt, DecodeJwt};
use ic_actix::{ICResult, ICError, AppStatePostgres, ToCookie, get_bearer_jwt};

use crate::postgres_helpers::{get_account_data};

pub async fn refresh_jwt(
		postgres: Data<AppStatePostgres>,
		request: HttpRequest,
	) -> ICResult<HttpResponse> {

	// Grab the Bearer header & check it's encoding
	let jwt_string = get_bearer_jwt(&request)?;

	// Decode the JWT & make sure it's ours
	let Ok(rjwt) = RefreshJwt::decode_with_defaults(jwt_string) else {
		return Err(ICError::panic("JWT decode"));
	};

	let account_id = rjwt.sub;
	let Ok(mut p_conn) = postgres.get_conn().await else {
		return Err(ICError::POSTGRES_CONN);
	};

	let Ok(account_data) = get_account_data(&mut p_conn, account_id).await else {
		// Should never happen, we just returned the id
		return Err(ICError::POSTGRES_CONN);
	};

	let Some(account_data) = account_data else {
		return Err(ICError::panic("Missing data"));
	};
	
	let rjwt:RefreshJwt = RefreshJwt::new_with_defaults(account_id);
	let ajwt:AuthJwt = AuthJwt::new_with_defaults(
		account_data.account_id,
		account_data.username.as_ref(),
		&account_data.claims,
	);

	// Return both JWTs and the data of the auth JWT in the body
	return Ok(HttpResponse::Ok()
		.cookie(rjwt.to_cookie())
		.cookie(ajwt.to_cookie())
		.finish());
}
