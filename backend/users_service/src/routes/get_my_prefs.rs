//use std::collections::HashMap;
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest};

//use crate::libredis::AppStateRedis;
use crate::db_objects::{AccountPreferences, get_account_preferences};
//use crate::login_helpers::{get_bearer_auth};
use ic_jwt::{AuthJwt, DecodeJwt};
use ic_actix::{ICError, ICResult, AppStatePostgres, get_bearer_jwt};


// This is used on the /profile page, to populate settings
pub async fn get_my_prefs(
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<Json<AccountPreferences>> {

	// TODO: Rate-limiter

	// Grab the Bearer header & check it's encoding
	let jwt_string = get_bearer_jwt(&request)?;

	// Decode the JWT & make sure it's ours
	let Ok(ajwt) = AuthJwt::decode_with_defaults(jwt_string) else {
		return Err(ICError::HEADER_VALIDATION);
	};

	// NOTE: This function/route can only fetch a user's own account preferences.
	// Mods, Admins, etc. need their own, and also people just browsing (w/ vis checks)
	let p_conn = postgres.get_conn().await?;
	let Ok(prefs) = get_account_preferences(&p_conn, ajwt.sub).await else {
		return Err(ICError::POSTGRES_CONN);
	};

	return Ok(Json(prefs));
}