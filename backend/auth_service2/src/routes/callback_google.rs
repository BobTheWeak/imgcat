//use std::collections::HashMap;
use serde::{Deserialize};
use actix_web::web::{Query, Data};
use actix_web::{HttpRequest, HttpResponse};
use openidconnect::reqwest;
use openidconnect::{
	AccessTokenHash,
	AuthorizationCode,
	AuthUrl,
	ClientId,
	ClientSecret,
	IssuerUrl,
	Nonce,
	OAuth2TokenResponse,
	PkceCodeVerifier,
	RedirectUrl,
	TokenResponse,
	TokenUrl,
};
use openidconnect::core::{
	CoreClient,
	CoreProviderMetadata,
};

use ic_actix::{ICError, ICResult, AppStatePostgres, AppStateRedis};
//use ic_actix::{get_country_code};

//use crate::provider_google::getage;
use crate::common_callback::handle_callback;

//use crate::app_state::AppStateProviders;
use crate::redis_helpers::get_login;


#[derive(Debug, Deserialize)]
pub struct TokenResponseParams {
	iss: String,
	code: String,
	state: String,
	// There are more fields, but we don't need them
}


pub async fn callback_google(
		params: Query<TokenResponseParams>,
		//metadata: Data<AppStateProviders>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		//request: HttpRequest,
	) -> ICResult<HttpResponse> {

	//let data = metadata.get("google").unwrap();

	// Verify URL Issuer:
	if params.iss != "https://accounts.google.com/" {
		return Err(ICError::HEADER_VALIDATION);
	}

	// TODO: Verify more headers

	// Grab the country_code from the Cloudflare header
	// Only used on sign-up, but the validation is useful
	//let country_code:Option<&str> = get_country_code(request.headers())?;

	// Grab the state key from redis
	let mut r_conn = redis.get_conn()?;
	let Ok(login_state) = get_login(&mut r_conn, &params.state) else {
		return Err(ICError::REDIS_CONN);
	};

	// Unpack the variables into openid structures
	let nonce = Nonce::new(login_state.nonce.clone());
	let pkce_v = PkceCodeVerifier::new(login_state.pkce_v.clone());

	// Create the HTTP client actually making the request
	let http_client = reqwest::ClientBuilder::new()
		.redirect(reqwest::redirect::Policy::none())
		.build()
		.unwrap();

	let Ok(metadata) = CoreProviderMetadata::discover_async(
		IssuerUrl::new("https://accounts.google.com".to_string()).unwrap(),
		&http_client,
	).await else {
		return Err(ICError::panic("Provider, discovery"));
	};

	let client = CoreClient::from_provider_metadata(
		metadata,
		ClientId::new(std::env::var("IC_OAUTH_GOOGLE_ID").unwrap()),
		Some(ClientSecret::new(std::env::var("IC_OAUTH_GOOGLE_SECRET").unwrap())),
		)
		.set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap())
		.set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
		.set_redirect_uri(RedirectUrl::new(std::env::var("IC_ORIGIN").unwrap()+"/auth/cb/google").unwrap())
		;

	// Contains all the configuration bits
	//let client = CoreClient::new(
	//		ClientId::new(std::env::var("IC_OAUTH_GOOGLE_ID").unwrap()),
	//		IssuerUrl::new("https://accounts.google.com".to_string()).unwrap(),
	//		JsonWebKeySetUrl::new("https://www.googleapis.com/oauth2/v3/certs".to_string()).unwrap(),
	//	)
	//	.set_client_secret(ClientSecret::new(std::env::var("IC_OAUTH_GOOGLE_SECRET").unwrap()))
	//	.set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap())
	//	.set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
	//	.set_redirect_uri(RedirectUrl::new(std::env::var("IC_ORIGIN").unwrap()+"/auth/cb/google").unwrap());

	// Ok, try to swap an auth token for an access token
	let Ok(token_response) = client
		.exchange_code(AuthorizationCode::new(params.code.clone()))
		.set_pkce_verifier(pkce_v)
		.request_async(&http_client).await else {
			return Err(ICError::panic("Provider exchange"));
			//return HttpResponse::ServiceUnavailable() // 503
			//	.insert_header(("IC-Error","Provider exchange")).finish();
		};

	// A giant block of validation
	let access_token = token_response.access_token();
	let id_token = token_response.id_token().unwrap();
	let id_token_verifier = client.id_token_verifier();
	let Ok(claims) = id_token.claims(&id_token_verifier, &nonce) else {
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
	
	// Call the UserInfo endpoint to grab the subject id
	//let id_response = match http_client
	//	.get("https://openidconnect.googleapis.com/v1/userinfo")
	//	.bearer_auth(access_token.secret())
	//	.send().await {
	//		Ok(v) => v,
	//		Err(e) => {
	//			println!("Error from UserInfo query: {:?}", e);
	//			return Err(ICError::panic("Provider, user"));
	//		}
	//	};

	// Hash the values and compare the hashes, in order to prevent timing attacks
	if let Some(expected_access_token_hash) = claims.access_token_hash() {
		let actual_access_token_hash = AccessTokenHash::from_token(
			access_token,
			id_token.signing_alg().unwrap(),
			id_token.signing_key(&id_token_verifier).unwrap(),
		).unwrap();
		if actual_access_token_hash != *expected_access_token_hash {
			return Err(ICError::error("Validation, hash"));
		}
	}

	// The ONLY thing we grab is Sub.
	let subject = claims.subject().as_str();

	// Connect to the DB
	let mut p_conn = postgres.get_conn().await?;

	// Do all the common functionality
	return handle_callback(&mut p_conn, &r_conn, "microsoft", &subject, access_token.secret(), login_state.redirect).await;
}
