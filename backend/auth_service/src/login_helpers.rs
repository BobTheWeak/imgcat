use crate::libredis::AppStateRedis;
use crate::libpostgres::AppStatePostgres;
use crate::helper_error::{HelperResult, HelperError};

use actix_web::{HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::cookie::{Cookie, SameSite, time::Duration};

use crate::libjwt::{RefreshJwt, AuthJwt, SignupJwt, EncodeJwt};


async fn is_user_banned(account_id:i64, redis:&AppStateRedis, pg:&AppStatePostgres) -> HelperResult<bool> {
	// Check if they're banned in Redis (temporarially)
	let Ok(is_banned_temp) = redis.is_user_banned(account_id) else {
		return Err(HelperError::new(500, "Check ban, Redis").into());
	};

	// If they're banned temporarially, don't bother checking the DB
	// NOTE: We do this for speed, but we may want to reverse this
	// because users would want to know about a permanent ban first.
	// ie. "OK I'll wait a week. Oh no, it's permanent!? HULK MAD!"
	if is_banned_temp {return Ok(is_banned_temp)}

	// Check if they're banned in Postgres (permanently)
	// TODO:
	let _ = pg;

	// fallthrough
	return Ok(is_banned_temp);
}

pub async fn get_refresh_jwt(account_id:i64, redis:&AppStateRedis, pg:&AppStatePostgres) -> HelperResult<RefreshJwt> {
	if is_user_banned(account_id, redis, pg).await? == true {
		return Err(HelperError::new(403, "Banned").into());
	};

	return Ok(RefreshJwt::new(account_id));
}

pub async fn get_auth_jwt(jwt:&RefreshJwt, redis:&AppStateRedis, pg:&AppStatePostgres) -> HelperResult<AuthJwt> {
	if is_user_banned(jwt.sub, redis, pg).await? == true {
		return Err(HelperError::new(403, "Banned").into());
	};

	let Ok(d) = pg.get_account_data(jwt.sub).await else {
		return Err(HelperError::new(500, "Postgres connection").into());
	};

	if let Some(d) = d {
		// Create & encode the Auth cookie
		return Ok(AuthJwt::new(
			jwt.sub,
			d.username.as_ref(),
			&d.claims,
		));
	} else {
		return Err(HelperError::new(500, "Postgres fetch acct").into());
	}
}


#[inline]
pub fn get_auth_cookie(ajwt:&AuthJwt) -> Cookie {
	Cookie::build("ic_auth", ajwt.encode().unwrap())
		.path("/")
		.secure(true)
		.http_only(true)
		.same_site(SameSite::Lax) // TODO: Strict isn't working
		.max_age(Duration::minutes(5))
		.finish()
}

#[inline]
pub fn get_refresh_cookie(rjwt:&RefreshJwt) -> Cookie {
	Cookie::build("ic_refresh", rjwt.encode().unwrap())
		.path("/")
		.secure(true)
		.http_only(true)
		.same_site(SameSite::Lax) // TODO: Strict isn't working
		.max_age(Duration::weeks(2))
		.finish()
}

#[inline]
pub fn get_signup_cookie(sjwt:&SignupJwt) -> Cookie {
	Cookie::build("ic_signup", sjwt.encode().unwrap())
		.path("/signup")
		.secure(true)
		.http_only(true)
		.same_site(SameSite::Lax) // TODO: Strict isn't working
		.max_age(Duration::minutes(15))
		.finish()
}


pub fn send_redirect(redirect_url:Option<String>, rjwt:Option<&RefreshJwt>, ajwt:Option<&AuthJwt>, sjwt:Option<&SignupJwt>) -> HttpResponse {
	let mut result = HttpResponseBuilder::new(StatusCode::TEMPORARY_REDIRECT); //307
	
	result.insert_header(("Location",
		redirect_url
		.or(std::env::var("IC_ORIGIN").ok())
		.expect("Could not parse envvar: IC_ORIGIN")));

	if let Some(rjwt) = rjwt {
		result.cookie(get_refresh_cookie(rjwt));
	}

	if let Some(ajwt) = ajwt {
		result.cookie(get_auth_cookie(ajwt));
	}

	if let Some(sjwt) = sjwt {
		result.cookie(get_signup_cookie(sjwt));
	}

	return result.finish();
}