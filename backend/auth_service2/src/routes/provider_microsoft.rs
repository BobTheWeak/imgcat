//use std::collections::HashMap;

use serde::{Serialize, Deserialize};
//use actix_web::{HttpRequest};
use actix_web::web::{Data, Query, Redirect};
//use ic_jwt::{AuthJwt, DecodeJwt};
//use ic_actix::{AppStateRedis};
use ic_actix::{ICError, ICResult, AppStateRedis};
//use ic_actix::{get_bearer_jwt};
use crate::redis_helpers::{LoginState, set_login};

//use mysql::prelude::Queryable;

//use crate::db_objects::comment::Comment;
//use crate::redis_helpers;
use oauth2::{AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::BasicClient;

#[derive(Serialize, Deserialize)]
pub(crate) struct ExpectedParams {
	#[serde(rename="r")]
	redirect_url:Option<String>
}

static BASE_URL:&str = "https://login.microsoftonline.com/common/oauth2/v2.0";

pub async fn provider_microsoft(
		params: Query<ExpectedParams>,
		redis: Data<AppStateRedis>,
		//request: HttpRequest,
	) -> ICResult<Redirect> {

	// NOTE: We can't use the OpenIDConnect library because it's too opinionated.
	// OAuth2 is the lower level library, which allows us to adjust more knobs and dials.
	let client = BasicClient::new(ClientId::new(std::env::var("IC_OAUTH_MICROSOFT_ID").unwrap()))
		.set_auth_uri(AuthUrl::new(BASE_URL.to_owned() + "/authorize").unwrap())
		.set_token_uri(TokenUrl::new(BASE_URL.to_owned() + "/token").unwrap())
		.set_redirect_uri(RedirectUrl::new(std::env::var("IC_ORIGIN").unwrap()+"/auth/cb/microsoft").unwrap());

	// Generate the PKCE authorization URL
	let (pkce_c, pkce_v) = PkceCodeChallenge::new_random_sha256();
	let nonce:String = rand::random::<u64>().to_string();

	let authorizer = client
		.authorize_url(CsrfToken::new_random)
		.add_scope(Scope::new("openid".to_string()))
		.add_extra_param("nonce", &nonce)
		.set_pkce_challenge(pkce_c);


	// // // // // // // // // // // // // // // // // // // //
	// BEGIN Legal age validation block
	// // // // // // // // // // // // // // // // // // // //
	// TODO: Move this to the sign-up logic entirely. This is an extra
	// DB check that doesn't need to happen for logins. It's just a
	// convenient thing to ask perms once, but TBH it's a better flow
	// to ask on the signup page, b/c we can explain *why* we're asking
	// for someone's age.
	// It should be a form:
	//    Welcome to ImgCat
	//    Username: [____________]
	//    [_] Accept Terms
	//    We must perform an additional age check, using the birthday of
	//    this account. It's a one-time check, and the value is not stored.
	//    _back_        <Verify your age>
	// When that's complete, flash a success message and change the button:
	//    _back_        <Create account>

	// In some jurisdictions, we may have to ask a user's age...
	// Check for the existance of the "CF-IPCountry" header (trust Cloudflare to do this for us)
	// NOTE: XX is an unknown value, T1 comes from the Tor network
	//if let Some(country_code) = request.headers().get("CF-IPCountry") {
	//	let Ok(country_code) = country_code.to_str() else {
	//		// This parsing could fail if the header has invalid characters
	//		return HttpResponse::BadRequest() // 400
	//			.insert_header(("IC-Error","Header validation")).finish();
	//	};
	//
	//	// Block Tor traffic. There are good reasons to allow it, but we need
	//	// to ensure our security against spam & hackers is rock-solid first
	//	if country_code == "T1" {
	//		return HttpResponse::Forbidden() // 403
	//			.insert_header(("IC-Error","Tor traffic is not allowed")).finish();
	//	}
	//
	//	if country_code != "XX" {
	//		// NOTE: As of today, the UK, AU & BR need validation
	//		// TODO: I'm not sure how to get the state-code... For now, pass None.
	//		let Ok(needed) = postgres.is_age_needed_on_signup(country_code, None).await else {
	//			return HttpResponse::ServiceUnavailable() // 503
	//				.insert_header(("IC-Error","Postgres connection")).finish();
	//		};
	//
	//		if needed {
	//			// OK... the user is from a juristiction that requires age verification
	//			// Upgrade our request and ask for more than the basic (implied) "openid" scope
	//			for claim in data.age_claims.iter() {
	//				authorizer = authorizer.add_scope(Scope::new(claim.to_string()));
	//			}
	//		}
	//	}
	//}

	// // // // // // // // // // // // // // // // // // // //
	// END Legal age validation block
	// // // // // // // // // // // // // // // // // // // //


	// After all that... grab the URL and proceed
	let (auth_url, csrf_token) = authorizer.url();

	// Create a wrapper object to hold the secret strings
	let login_state = LoginState {
		state: csrf_token.secret().to_string(),
		nonce: nonce,
		pkce_v: pkce_v.secret().to_string(),
		redirect: params.redirect_url.clone()
	};

	// Store all of the secret bits in Redis
	let mut r_conn = redis.get_conn()?;
	set_login(&mut r_conn, &login_state)?;

	// Redirect to the URL that the provider asked us to do
	return Ok(Redirect::to(auth_url.to_string()));
	//return HttpResponse::TemporaryRedirect()
	//	.insert_header(("Location", auth_url.to_string()))
	//	.finish();
}
