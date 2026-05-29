use serde::Deserialize;
use actix_web::get;
use actix_web::web::{Path, Query, Data};
use actix_web::{HttpRequest, HttpResponse};
use openidconnect::core::{
	CoreAuthenticationFlow,
	CoreClient,
};
use openidconnect::{
	ClientId,
	ClientSecret,
	CsrfToken,
	Nonce,
	PkceCodeChallenge,
	RedirectUrl,
	Scope,
};

use crate::app_state::{AppStateProviders};
use crate::libpostgres::{AppStatePostgres};
use crate::libredis::{AppStateRedis, LoginState};

#[derive(Debug, Deserialize)]
struct AuthParams {
	#[serde(rename="r")]
	redirect_url: Option<String>
}

#[get("/{provider}")]
pub async fn provider(
		path: Path<String>,
		params: Query<AuthParams>,
		metadata: Data<AppStateProviders>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
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
				.expect("Could not parse envvar: IC_ORIGIN"),
			&provider
		)
	).unwrap());

	// Generate the PKCE authorization URL
	let (pkce_c, pkce_v) = PkceCodeChallenge::new_random_sha256();
	let mut authorizer = client.authorize_url(
		CoreAuthenticationFlow::AuthorizationCode,
		CsrfToken::new_random,
		Nonce::new_random,
	)
	// NOTE: openid is always implied, and sent automatically,
	// setting it here would send "scope=openid+openid"
	//.add_scope(Scope::new("openid".to_string()))
	.set_pkce_challenge(pkce_c);


	// // // // // // // // // // // // // // // // // // // //
	// BEGIN Legal age validation block
	// // // // // // // // // // // // // // // // // // // //
	// TODO: Move this to the sign-up logic entirely. This is an extra
	// DB check that doesn't need to happen for logins. It's just a
	// convenient thing to ask perms once, but no... This is too early
	// and we can't explain why we need it, yet.

	// In some jurisdictions, we may have to ask a user's age...
	// Check for the existance of the "CF-IPCountry" header (trust Cloudflare to do this for us)
	// NOTE: XX is an unknown value, T1 comes from the Tor network
	if let Some(country_code) = request.headers().get("CF-IPCountry") {
		let Ok(country_code) = country_code.to_str() else {
			// This parsing could fail if the header has invalid characters
			return HttpResponse::BadRequest() // 400
				.insert_header(("IC-Error","Header validation")).finish();
		};
	
		// Block Tor traffic. There are good reasons to allow it, but we need
		// to ensure our security against spam & hackers is rock-solid first
		if country_code == "T1" {
			return HttpResponse::Forbidden() // 403
				.insert_header(("IC-Error","Tor traffic is not allowed")).finish();
		}
	
		if country_code != "XX" {
			// NOTE: As of today, the UK, AU & BR need validation
			// TODO: I'm not sure how to get the state-code... For now, pass None.
			if let Ok(needed) = postgres.is_age_needed_on_signup(country_code, None).await {
				if needed {
					// OK... the user is from a juristiction that requires age verification
					// Upgrade our request and ask for more than the basic (implied) "openid" scope
					for claim in data.age_claims.iter() {
						authorizer = authorizer.add_scope(Scope::new(claim.to_string()));
					}
				}
			} else {
				return HttpResponse::InternalServerError() // 500
					.insert_header(("IC-Error","Postgres connection")).finish();
			};
		}
	}

	// // // // // // // // // // // // // // // // // // // //
	// END Legal age validation block
	// // // // // // // // // // // // // // // // // // // //


	// After all that... grab the URL and proceed
	let (auth_url, csrf_token, nonce) = authorizer.url();

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
			.insert_header(("IC-Error","Redis"))
			.finish();
	};

	// Redirect to the URL that the provider asked us to do
	return HttpResponse::TemporaryRedirect()
		.insert_header(("Location", auth_url.to_string()))
		.finish();
}
