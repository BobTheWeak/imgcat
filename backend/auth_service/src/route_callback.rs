//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::get;
use actix_web::web::{Path, Query, Data};
use actix_web::{HttpRequest, HttpResponse};
use openidconnect::reqwest;
use openidconnect::core::{CoreClient};
use openidconnect::{
	AccessTokenHash,
	AuthorizationCode,
	ClientId,
	ClientSecret,
	Nonce,
	OAuth2TokenResponse,
	PkceCodeVerifier,
	RedirectUrl,
	TokenResponse,
};

use crate::provider_google;

use crate::app_state::AppStateProviders;
use crate::libredis::AppStateRedis;
use crate::ic_postgres::AppStatePostgres;
use crate::libjwt::{RefreshJwt, AuthJwt, SignupJwt};
use crate::login_helpers::{get_refresh_jwt, get_auth_jwt, send_redirect};
use crate::age_verification::AgeVerification;

#[derive(Debug, Deserialize)]
struct TokenResponseParams {
	iss: String,
	code: String,
	state: String,
	// There are more fields, but we don't need them
}

//#[routes]
#[get("/{provider}/callback")]
//#[get("/api/auth/{provider}/callback")]
pub async fn callback(
		path: Path<String>,
		params: Query<TokenResponseParams>,
		metadata: Data<AppStateProviders>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> HttpResponse {

	let provider:String = path.into_inner();
	let Some(data) = metadata.get(&provider) else {
		return HttpResponse::NotFound().finish();
	};

	// Verify URL Issuer:
	if params.iss != data.issuer_url {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header validation")).finish();
	}

	// TODO: Verify more headers

	// Grab the country_code from the Cloudflare header
	// Only used on sign-up, but the validation is useful
	let country_code:Option<&str> = {
		if let Some(cc) = request.headers().get("CF-IPCountry") {
			let Ok(cc) = cc.to_str() else {
				// This parsing could fail if the header has invalid characters
				return HttpResponse::Unauthorized() // 401
					.insert_header(("IC-Error","Header")).finish();
			};

			// Block Tor traffic. There are good reasons to allow it, but we need
			// to ensure our security against spam & hackers is rock-solid first
			if cc == "T1" {
				return HttpResponse::Forbidden() // 403
					.insert_header(("IC-Error","Tor traffic is not allowed")).finish();
			}
			
			if cc != "XX" {
				Some(cc)
			} else {
				None
			}
		} else {
			None
		}
	};

	// Grab the state key from redis
	let Ok(redis_key) = redis.get_login(&params.state) else {
		return HttpResponse::ServiceUnavailable() // 503
			.insert_header(("IC-Error","Redis connection")).finish();
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
				.expect("Could not parse envvar: IC_ORIGIN"),
			&provider,
		)
	).unwrap());

	// Ok, try to swap an auth token for an access token
	let Ok(token_response) = client
		.exchange_code(AuthorizationCode::new(params.code.clone())).unwrap()
		.set_pkce_verifier(pkce_v)
		.request_async(&http_client).await else {
			return HttpResponse::ServiceUnavailable() // 503
				.insert_header(("IC-Error","Provider exchange")).finish();
		};

	// A giant block of validation
	let access_token = token_response.access_token();
	let id_token = token_response.id_token().unwrap();
	let id_token_verifier = client.id_token_verifier();
	let Ok(claims) = id_token.claims(&id_token_verifier, &nonce) else {
		return HttpResponse::ServiceUnavailable() // 503
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
			return HttpResponse::ServiceUnavailable() // 503
				.insert_header(("IC-Error","Validation, hash")).finish();
		}
	}
	
	// Now grab the subject. This is what all the marbles are for...
	let subject = claims.subject().as_str();

	// Grab the account_id
	let Ok(account_id) = postgres.get_account_id(&provider, &subject).await else {
		return HttpResponse::ServiceUnavailable() // 503
			.insert_header(("IC-Error","Postgres, connection error")).finish();
	};

	// Check if we already have an account
	if let Some(account_id) = account_id {
		// They have an account!

		let rjwt:RefreshJwt = match get_refresh_jwt(account_id, &redis, &postgres).await {
			Ok(v) => v, Err(e) => return e.into()
		};
		let ajwt:AuthJwt = match get_auth_jwt(&rjwt, &redis, &postgres).await {
			Ok(v) => v, Err(e) => return e.into()
		};
		return send_redirect(redirect_url, Some(&rjwt), Some(&ajwt), None);

	} else {
		// No account. Redirect them to the new account creation screen.
		
		// If we need to check their age, do this now.
		let age_ver:Option<AgeVerification> = {
			if let Some(country_code) = country_code {
				// TODO: I'm not sure how to get the state-code... For now, pass None.
				let Ok(needed) = postgres.is_age_needed_on_signup(country_code, None).await else {
					return HttpResponse::ServiceUnavailable() // 503
						.insert_header(("IC-Error","Postgres connection")).finish();
				};

				if !needed {
					// Most common path - No age-verification needed
					None
				} else {
					// Upgrade our request and ask for more than the basic (implied) "openid" scope
					
					// NOTE: I tried to put an async fn ptr into the metadata object
					// we pass around, but it was getting *really* nasty. So easy mode:
					let age:Option<u16> = match provider.as_ref() {
						"google" => provider_google::getage(access_token.secret()).await,
						_ => unimplemented!("Provider is missing from match block"),
					};

					// NOTE: If they deny permission, we return age=0
					// TODO: At the moment, we have no way of determining the user's state
					Some(AgeVerification::new (country_code, None, age.unwrap_or(0_u16)))
				}
			} else {
				None // Cloudflare can't pinpoint their location (rare)
			}
		};

		// Give them a cookie with provider info, so we can apply that on signup
		let sjwt = SignupJwt::new(&provider, subject, age_ver);
		return send_redirect(Some("/signup".to_string()), None, None, Some(&sjwt));
	}
}
