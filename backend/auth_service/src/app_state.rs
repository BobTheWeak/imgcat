use std::collections::HashMap;

use openidconnect::reqwest;
use openidconnect::core::{CoreProviderMetadata};
use openidconnect::{IssuerUrl};


pub type AppStateProviders = HashMap<String, AppStateItem>;
pub struct AppStateItem {
	pub issuer_url: String,
	pub client_id: String,
	pub client_secret: String,
	pub age_claims: Vec<String>,
	pub metadata: CoreProviderMetadata,
}

impl AppStateItem {
	pub async fn new(issuer_url:&str, env_prefix:&str, age_claims:Vec<String>) -> Self {
		
		let http_client = reqwest::ClientBuilder::new()
			.redirect(reqwest::redirect::Policy::none())
			.build()
			.unwrap();

		let metadata = CoreProviderMetadata::discover_async(
			IssuerUrl::new(issuer_url.to_string()).unwrap(),
			&http_client,
		).await.expect("Could not load OpenID provider configuration");

		Self {
			issuer_url: issuer_url.to_string(),
			client_id: std::env::var(env_prefix.to_owned() + "_ID")
				.expect(&("Could not load envvar ".to_owned() + env_prefix + "_ID")),
			client_secret: std::env::var(env_prefix.to_owned() + "_SECRET")
				.expect(&("Could not load envvar ".to_owned() + env_prefix + "_SECRET")),
			age_claims: age_claims.clone(),
			metadata,
		}
	}
}