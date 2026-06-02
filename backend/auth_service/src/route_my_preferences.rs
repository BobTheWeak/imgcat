//use std::collections::HashMap;
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest};

//use crate::libredis::AppStateRedis;
use crate::ic_postgres::{AppStatePostgres, AccountPreferences};
use crate::login_helpers::{get_bearer_auth};
use crate::libjwt::{AuthJwt, DecodeJwt};
use crate::ic_error::{ICError, ICResult};


// This is used on the /profile page, to populate settings
pub async fn get_my_prefs(
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<Json<AccountPreferences>> {

	// TODO: Rate-limiter

	// Grab the Bearer header & check it's encoding
	let jwt_string = get_bearer_auth(&request)?;

	// Decode the JWT & make sure it's ours
	let Ok(ajwt) = AuthJwt::decode(jwt_string) else {
		return Err(ICError::HEADER_VALIDATION);
	};

	// NOTE: This function/route can only fetch a user's own account preferences.
	// Mods, Admins, etc. need their own, and also people just browsing (w/ vis checks)
	let Ok(prefs) = postgres.get_account_preferences(ajwt.sub).await else {
		return Err(ICError::POSTGRES_ERROR);
	};

	return Ok(Json(prefs));
}
