//use std::collections::HashMap;
use actix_web::web::{Data, Form, Json};
use actix_web::{HttpRequest};

//use crate::libredis::AppStateRedis;
use crate::db_objects::{AccountPreferencesSetter, set_account_preferences};
//use crate::login_helpers::{get_bearer_auth};
use ic_jwt::{AuthJwt, DecodeJwt};
use ic_actix::{ICError, ICResult, AppStatePostgres, get_bearer_jwt};


pub async fn set_my_prefs(
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		form: Form<AccountPreferencesSetter>,
		request: HttpRequest,
	) -> ICResult<Json<i32>> {

	// TODO: Rate-limiter

	// Grab the Bearer header & check it's encoding
	let jwt_string = get_bearer_jwt(&request)?;

	// Decode the JWT & make sure it's ours
	let Ok(ajwt) = AuthJwt::decode_with_defaults(jwt_string) else {
		return Err(ICError::HEADER_VALIDATION);
	};

	// verify the form account_id matches the JWT
	if form.account_id != ajwt.sub {
		return Err(ICError::HEADER_VALIDATION);
	}

	// NOTE: This function/route can only fetch a user's own account preferences.
	// Mods, Admins, etc. need their own, and also people just browsing (w/ vis checks)
	let p_conn = postgres.get_conn().await?;
	return Ok(Json(set_account_preferences(&p_conn, &form).await?));
}