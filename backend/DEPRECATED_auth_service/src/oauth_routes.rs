//use std::collections::HashMap;
use serde::Deserialize;
//#[allow(unused_imports)] // It gets confused & throws warnings
use actix_web::{web, get, post, routes};
use actix_web::{HttpResponse};
use openidconnect::reqwest;
use openidconnect::core::{
	CoreAuthenticationFlow,
	CoreClient,
	//CoreProviderMetadata,
};
use openidconnect::{
	AccessTokenHash,
	AuthorizationCode,
	ClientId,
	ClientSecret,
	CsrfToken,
	//IssuerUrl,
	Nonce,
	OAuth2TokenResponse,
	PkceCodeChallenge,
	PkceCodeVerifier,
	RedirectUrl,
	Scope,
	TokenResponse,
};

use crate::app_state::{AppStateDictionary};
use crate::libredis::{AppStateRedis, LoginState};
use crate::libpostgres;

//
//pub type AppStateDictionary = HashMap<String, AppStateItem>;
//pub struct AppStateItem {
//	issuer_url: String,
//	client_id: String,
//	client_secret: String,
//	metadata: CoreProviderMetadata,
//}
//
//impl AppStateItem {
//	pub async fn new(issuer_url:&str, env_prefix:&str) -> Self {
//		
//		let http_client = reqwest::ClientBuilder::new()
//			.redirect(reqwest::redirect::Policy::none())
//			.build()
//			.unwrap();
//
//		let metadata = CoreProviderMetadata::discover_async(
//			IssuerUrl::new(issuer_url.to_string()).unwrap(),
//			&http_client,
//		).await.expect("Could not load OpenID provider configuration");
//
//		Self {
//			issuer_url: issuer_url.to_string(),
//			client_id: std::env::var(env_prefix.to_owned() + "_ID")
//				.expect(&("Could not load envvar ".to_owned() + env_prefix + "_ID")),
//			client_secret: std::env::var(env_prefix.to_owned() + "_SECRET")
//				.expect(&("Could not load envvar ".to_owned() + env_prefix + "_SECRET")),
//			metadata,
//		}
//	}
//}


#[derive(Debug, Deserialize)]
struct AuthParams {
	#[serde(rename="r")]
	redirect_url: Option<String>
}

#[routes]
#[get("/{provider}")]
#[get("/api/auth/{provider}")] // Unproxied testing
pub async fn auth(
		path: web::Path<String>,
		params: web::Query<AuthParams>,
		metadata: web::Data<AppStateDictionary>,
		//postgres: web::Data<libpostgres::AppStatePostgres>,
		redis: web::Data<AppStateRedis>,
	) -> HttpResponse {

	// Grab the provider & pull the pre-fetched discovery data
	let provider:String = path.into_inner();
	let Some(data) = metadata.get(&provider) else {
		return HttpResponse::NotFound().finish();
	};

	// Create the core OpenID request factory object
	let client = CoreClient::from_provider_metadata(
		data.metadata.clone(),
		ClientId::new(data.client_id.clone()),
		Some(ClientSecret::new(data.client_secret.clone())),
	)
	.set_redirect_uri(RedirectUrl::new(
		format!("{}/api/auth/{}/callback",
			std::env::var("IC_ORIGIN")
				.unwrap_or("Could not parse envvar: IC_ORIGIN"),
			&provider
		)
	).unwrap());

	// Generate the PKCE authorization URL
	let (pkce_c, pkce_v) = PkceCodeChallenge::new_random_sha256();
	let (auth_url, csrf_token, nonce) = client.authorize_url(
		CoreAuthenticationFlow::AuthorizationCode,
		CsrfToken::new_random,
		Nonce::new_random,
	)
	// NOTE: openid is already sent & already implied
	//.add_scope(Scope::new("openid".to_string()))
	//.add_scope(Scope::new("https://www.googleapis.com/auth/user.birthday.read".to_string()))
	.set_pkce_challenge(pkce_c)
	.url();
	
	// Create a wrapper object to hold the secret strings
	let login_state = LoginState {
		state: csrf_token.secret().to_string(),
		nonce: nonce.secret().to_string(),
		pkce_v: pkce_v.secret().to_string(),
		redirect: params.redirect_url.clone()
	};

	// Store all of the secret bits in Redis
	if let Err(_) = redis.set_login(&login_state) {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Redis")).finish();
	};

	// Redirect to the URL that the provider asked us to do
	return HttpResponse::Ok()
		.body(auth_url.to_string());
}


#[derive(Debug, Deserialize)]
struct TokenResponseParams {
	iss: String,
	code: String,
	state: String,
	// Google returns more fields, but we don't need them
}

#[routes]
#[get("/{provider}/callback")]
#[get("/api/auth/{provider}/callback")] // Unproxied testing
pub async fn callback(
		path: web::Path<String>,
		params: web::Query<TokenResponseParams>,
		metadata: web::Data<AppStateDictionary>,
		postgres: web::Data<libpostgres::AppStatePostgres>,
		redis: web::Data<AppStateRedis>,
	) -> HttpResponse {

	let provider:String = path.into_inner();
	let Some(data) = metadata.get(&provider) else {
		return HttpResponse::NotFound().finish();
	};

	// Verify URL Issuer:
	if params.iss != data.issuer_url {
		return HttpResponse::BadRequest()
			.insert_header(("IC-Error","Validation, issuer")).finish();
	}

	// TODO: Verify more headers

	// Grab the state key from redis
	let Ok(redis_key) = redis.get_login(&params.state) else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Redis")).finish();
	};

	// Unpack the variables into openid structures
	let nonce = Nonce::new(redis_key.nonce.clone());
	let pkce_v = PkceCodeVerifier::new(redis_key.pkce_v.clone());
	let redirect_url = redis_key.redirect.clone();

	// Create the HTTP client actually making the request
	let http_client = reqwest::ClientBuilder::new()
		.redirect(reqwest::redirect::Policy::none())
		.build()
		.unwrap();

	// Contains all the configuration bits
	let client = CoreClient::from_provider_metadata(
		data.metadata.clone(),
		ClientId::new(data.client_id.clone()),
		Some(ClientSecret::new(data.client_secret.clone())),
	)
	.set_redirect_uri(RedirectUrl::new(
		format!("{}/api/auth/{}/callback",
			std::env::var("IC_ORIGIN")
				.unwrap_or("Could not parse envvar: IC_ORIGIN"),
			&provider
		)
	).unwrap());

	// Ok, try to swap an auth token for an access token
	let Ok(token_response) = client
		.exchange_code(AuthorizationCode::new(params.code.clone())).unwrap()
		.set_pkce_verifier(pkce_v)
		.request_async(&http_client).await else {
			return HttpResponse::InternalServerError()
				.insert_header(("IC-Error","Provider, exchange")).finish();
		};

	// A giant block of validation
	let access_token = token_response.access_token();
	let id_token = token_response.id_token().unwrap();
	let id_token_verifier = client.id_token_verifier();
	let Ok(claims) = id_token.claims(&id_token_verifier, &nonce) else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Validation, claims")).finish();
	};
	
	// Hash the values and compare the hashes, in order to prevent timing attacks
	if let Some(expected_access_token_hash) = claims.access_token_hash() {
		let actual_access_token_hash = AccessTokenHash::from_token(
			access_token,
			id_token.signing_alg().unwrap(),
			id_token.signing_key(&id_token_verifier).unwrap(),
		).unwrap();
		if actual_access_token_hash != *expected_access_token_hash {
			return HttpResponse::InternalServerError()
				.insert_header(("IC-Error","Validation, hash")).finish();
		}
	}
	
	// Now grab the subject
	let subject = claims.subject().as_str();

	// Check if we already have an account
	if let Some(account_id) = postgres.get_account_id(&provider, &subject).await {
		// They have an account. Set auth cookies, and redirect to wherever they were originally.
		return HttpResponse::Ok().body(format!(
			"Account: {}\nSubject: {}\n",
			account_id.to_string(),
			subject,
		));
	} else {
		// No account. Redirect them to the new account creation screen.
		return HttpResponse::Ok().body(format!(
			"Account: {}\nSubject: {}\n",
			"- NULL -".to_string(),
			subject,
		));
	}
}
