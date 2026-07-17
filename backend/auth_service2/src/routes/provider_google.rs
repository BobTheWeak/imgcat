use serde::Deserialize;
use actix_web::web::{Query, Data, Redirect};
//use openidconnect::core::{
//	CoreAuthenticationFlow,
//	CoreClient,
//};
//use openidconnect::{
//	ClientId,
//	ClientSecret,
//	CsrfToken,
//	Nonce,
//	PkceCodeChallenge,
//	RedirectUrl,
//	//Scope,
//};
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::{BasicClient};

use ic_actix::{ICResult};


//use crate::app_state::{AppStateProviders};
use crate::redis_helpers::{LoginState, set_login};

use ic_actix::{AppStateRedis};

#[derive(Debug, Deserialize)]
pub struct AuthParams {
	#[serde(rename="r")]
	redirect_url: Option<String>
}

pub async fn provider_google(
		params: Query<AuthParams>,
		redis: Data<AppStateRedis>,
		//request: HttpRequest,
	) -> ICResult<Redirect> {

	// TODO: Rate-limit

	// NOTE: The OpenIDConnect library breaks for Microsoft OIDC, because it's too opinionated & "auto-magic".
	// OAuth2 is the lower level library, which allows us to adjust more knobs and dials.
	// Google works fine with openidconnect, but we can simplify libraries using *just* Oauth2
	let client = BasicClient::new(ClientId::new(std::env::var("IC_OAUTH_GOOGLE_ID").unwrap()))
		.set_client_secret(ClientSecret::new(std::env::var("IC_OAUTH_GOOGLE_SECRET").unwrap()))
		.set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap())
		.set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
		.set_redirect_uri(RedirectUrl::new(std::env::var("IC_ORIGIN").unwrap()+"/auth/cb/google").unwrap());

	// Generate the PKCE authorization URL
	let (pkce_c, pkce_v) = PkceCodeChallenge::new_random_sha256();
	let nonce:String = rand::random::<u64>().to_string();

	let (auth_url, csrf_token) = client
		.authorize_url(CsrfToken::new_random)
		.add_scope(Scope::new("openid".to_string()))
		.add_extra_param("nonce", &nonce)
		.set_pkce_challenge(pkce_c)
		.url();

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
