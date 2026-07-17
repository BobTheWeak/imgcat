//use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::{Data, Query};
//use ic_jwt::{AuthJwt, RefreshJwt, SignupJwt, DecodeJwt, EncodeJwt, AgeVerification};
//use ic_actix::{AppStateRedis, AppStatePostgres};
use ic_actix::{ICError, ICResult, AppStateRedis};
//use ic_actix::{get_bearer_jwt};

// TODO: Use ic_actix objects
use crate::AppStatePostgres;
use crate::redis_helpers::{LoginState, get_login};
use crate::common_callback::handle_callback;

//use mysql::prelude::Queryable;

//use crate::db_objects::comment::Comment;
//use crate::redis_helpers;
use oauth2::{RedirectUrl, AuthUrl, TokenUrl, ClientId, AuthorizationCode, TokenResponse, PkceCodeVerifier};
use oauth2::basic::BasicClient;
use oauth2::reqwest;

//use crate::postgres_helpers::{get_account_id, get_account_data};
//use deadpool_postgres::tokio_postgres::{Connection};


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ExpectedParams {
	code:String,
	state:String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExpectedUserInfo {
	sub:String,
	// Several more fields we don't use
}

// NOTE: This should be handled via discovery, but Microsoft returns the issuer
// URL with a replacement variable {tenantid}, which breaks automatic processes.
static BASE_URL:&str = "https://login.microsoftonline.com/common/oauth2/v2.0";
static USER_INFO_URL:&str = "https://graph.microsoft.com/oidc/userinfo";

pub async fn callback_microsoft(
		params: Query<ExpectedParams>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<HttpResponse> {

	// NOTE: We can't use the OpenIDConnect library because it's too opinionated.
	// OAuth2 is the lower level library, which allows us to adjust more knobs and dials.
	let client = BasicClient::new(ClientId::new(std::env::var("IC_OAUTH_MICROSOFT_ID").unwrap()))
		//.set_client_secret(ClientSecret::new(std::env::var("IC_OAUTH_MICROSOFT_SECRET").unwrap()))
		.set_auth_uri(AuthUrl::new(BASE_URL.to_owned() + "/authorize").unwrap())
		.set_token_uri(TokenUrl::new(BASE_URL.to_owned() + "/token").unwrap())
		// Not sure if this is needed. Keeping it just to keep the code identical
		.set_redirect_uri(RedirectUrl::new(std::env::var("IC_ORIGIN").unwrap()+"/auth/cb/microsoft").unwrap());

	// Get all of the secret bits we stashed in Redis
	let mut r_conn = redis.get_conn()?;
	let login_state:LoginState = get_login(&mut r_conn, &params.state)?;

	// Microsoft requires us to send the Origin header with the request
	let mut http_headers = reqwest::header::HeaderMap::new();
	http_headers.append("Origin", std::env::var("IC_ORIGIN").unwrap().parse().unwrap());

	// Restrict redirects to protect against SSRF
	let http_client = reqwest::ClientBuilder::new()
		.default_headers(http_headers)
		.redirect(reqwest::redirect::Policy::none())
		.build().unwrap();

	// Exchange the code for an access token
	let token_response = match client
		.exchange_code(AuthorizationCode::new(params.code.clone()))
		.set_pkce_verifier(PkceCodeVerifier::new(login_state.pkce_v))
		.request_async(&http_client)
		.await {
			Ok(v) => v,
			Err(e) => {
				println!("Error from provider: {:?}", e);
				return Err(ICError::panic("Provider, exchange"));
			}
		};

	// Extract the access_token, ignore the refresh_token, etc.
	let access_token:&String = token_response.access_token().secret();

	// Call the UserInfo endpoint to grab the subject id
	let id_response = match http_client
		.get(USER_INFO_URL)
		.bearer_auth(access_token)
		.send().await {
			Ok(v) => v,
			Err(e) => {
				println!("Error from UserInfo query: {:?}", e);
				return Err(ICError::panic("Provider, user"));
			}
		};

	// The ONLY thing we grab is Sub.
	let id_body = id_response.text().await.unwrap();
	let sub:String = match serde_json::from_str::<ExpectedUserInfo>(&id_body) {
		Ok(v) => v.sub,
		Err(_) => return Err(ICError::panic("Provider, body")),
	};

	// Connect to the DB
	let mut p_conn = postgres.get_conn().await?;

	// Do all the common functionality
	return handle_callback(&mut p_conn, &r_conn, "microsoft", &sub, access_token, login_state.redirect).await;

	/*

	// A giant block of validation
	let access_token = token_response.access_token();
	let id_token = token_response.id_token().unwrap();
	let id_token_verifier = client.id_token_verifier();
	let Ok(claims) = id_token.claims(&id_token_verifier, login_state.nonce) else {
		// NOTE: This shouldn't fail...
		// TODO: But it failed once when auth had been running for a few
		// weeks w/o rebuilding. I suspect the provider rotated their certs.
		// I rebuilt & redeployed, and that worked. But I suspect a redeploy
		// would have been enough. And obviously... we need to refresh our
		// cache every so often. This might be an indicator to tell the service
		// to flush it. But the user trying to log in needs to redo & try again.
		return Err(ICError::error("Validation claims")) // 400
		//return HttpResponse::ServiceUnavailable() // 503
		//	.insert_header(("IC-Error","Validation, claims")).finish();
	};
	
	// Hash the values and compare the hashes, in order to prevent timing attacks
	if let Some(expected_access_token_hash) = claims.access_token_hash() {
		let actual_access_token_hash = AccessTokenHash::from_token(
			access_token,
			id_token.signing_alg().unwrap(),
			id_token.signing_key(&id_token_verifier).unwrap(),
		).unwrap();
		if actual_access_token_hash != *expected_access_token_hash {
			return Err(ICError::error("Validation hash")) // 400
			//return HttpResponse::ServiceUnavailable() // 503
			//	.insert_header(("IC-Error","Validation, hash")).finish();
		}
	}
	
	// Now grab the subject. This is what all the marbles are for...
	let subject = claims.subject().as_str();

	let Ok(mut p_conn) = postgres.get_conn().await else {
		return Err(ICError::POSTGRES_CONN);
	};

	// Grab the account_id
	let account_id:Option<i64> = get_account_id(&mut p_conn, "microsoft", &subject).await?;

	// Check if we already have an account
	if let Some(account_id) = account_id {
		// They have an account!

		// Grab the data to create JWTs
		let Some(account_data) = get_account_data(&mut p_conn, account_id).await? else {
			// Should never happen, we just returned the id
			return Err(ICError::panic("Postgres data error"));
		};
		
		let rjwt:RefreshJwt = RefreshJwt::new_with_defaults(account_id);
		let ajwt:AuthJwt = AuthJwt::new_with_defaults(
			account_data.account_id,
			account_data.username.as_ref(),
			&account_data.claims,
		);

		return Ok(
			HttpResponse::TemporaryRedirect()
				.insert_header(("Location",
					login_state.redirect_url
					.or(std::env::var("IC_ORIGIN").ok())
					.expect("Could not parse envvar: IC_ORIGIN")))
				.cookie(rjwt.to_cookie())
				.cookie(ajwt.to_cookie())
				.finish()
		);
		//return Ok(send_redirect(redirect_url, Some(&rjwt), Some(&ajwt), None));

	} else {
		// No account. Redirect them to the new account creation screen.
		
		// If we need to check their age, do this now.
		//let age_ver:Option<AgeVerification> = {
		//	if let Some(country_code) = country_code {
		//		// TODO: I'm not sure how to get the state-code... For now, pass None.
		//		let Ok(needed) = is_age_needed_on_signup(&p_conn, country_code, None).await else {
		//			return Err(ICError::POSTGRES_CONN);
		//			//return HttpResponse::ServiceUnavailable() // 503
		//			//	.insert_header(("IC-Error","Postgres connection")).finish();
		//		};
		//
		//		if !needed {
		//			// Most common path - No age-verification needed
		//			None
		//		} else {
		//			let age:Option<u16> = getage(access_token.secret()).await;
		//
		//			// NOTE: If they deny permission, we return age=0
		//			// TODO: At the moment, we have no way of determining the user's state
		//			Some(AgeVerification::new (country_code, None, age.unwrap_or(0_u16)))
		//		}
		//	} else {
		//		None // Cloudflare can't pinpoint their location (rare)
		//	}
		//};

		// Give them a cookie with provider info, so we can apply that on signup
		let sjwt = SignupJwt::new_with_defaults("google", subject, None);

		return Ok(
			HttpResponse::TemporaryRedirect()
				.insert_header(("Location", "/signup"))
				.cookie(sjwt.to_cookie())
				.finish()
		);
		//return Ok(send_redirect(Some("/signup".to_string()), None, None, Some(&sjwt)));
	}
	*/
}
