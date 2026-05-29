mod libpostgres;
mod libredis;
mod libjwt;
mod app_state;
mod helper_error;
mod login_helpers;
mod age_verification;

// Generic functions
mod route_provider;
mod route_callback;
mod route_refresh;
mod route_namefree;
//mod route_agecheck;

// Account Management functions
mod route_create;

// Profider-specific functions (ie. age checks)
mod provider_google;

// Heath Check stuff
mod health_check;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::{Data};
use env_logger::Env;

use crate::libpostgres::{AppStatePostgres};
use crate::libredis::{AppStateRedis};
use crate::app_state::{AppStateProviders, AppStateItem};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(Env::default().default_filter_or("debug"));

	// Check for required ENVVARs
	let mut check = true;
	let required_envvars = vec![
		"IC_ORIGIN",
		"IC_JWT_PUB",
		"IC_JWT_PVT",
		"IC_REDIS_HOST",
		// Some of UDB vars could be defaulted
		"IC_UDB_HOST",
		"IC_UDB_PORT",
		"IC_UDB_DB",
		"IC_UDB_USER",
		"IC_UDB_PASS",
		// I hope all we need is ID/Secret for each provider
		"IC_OAUTH_GOOGLE_ID",
		"IC_OAUTH_GOOGLE_SECRET",
	];
	let optional_envvars = vec![
		"IC_JWT_PUB_ROTATED", // Default: None - If set, try a backup JWT decode
		"IC_JWT_ISS",  // Default: Use IC_ORIGIN envvar instead
		"IC_JWT_AUD",  // Default: Use IC_ORIGIN envvar instead
		"IC_REDIS_PORT", // Default: 8080
		"IC_REDIS_DB",   // Default: None
	];
	for s in required_envvars {if std::env::var(s).is_err(){println!("Error: missing envvar {}",s);check=false}}
	for s in optional_envvars {if std::env::var(s).is_err(){println!("Info: missing envvar {}, using default",s)}}
	if !check {return Ok(())}

	// Query all our providers for their OpenID discovery data
	let mut dict_generic = AppStateProviders::new();
	dict_generic.insert("google".to_string(),
		AppStateItem::new( // (ISS URL, ENVVAR prefix, scope to add for age verification)
			"https://accounts.google.com",
			"IC_OAUTH_GOOGLE",
			vec![
				"profile".to_string(),
				"https://www.googleapis.com/auth/user.birthday.read".to_string(),
			],
			).await);
	// TODO: Add more providers here

	// These use a Mutex, so the connection pool is shared across the whole app
	let shared_pg = Data::new(AppStatePostgres::new().await);
	let shared_redis = Data::new(AppStateRedis::new());
	// These are just cloned to every thread, as needed
	let shared_metadata = Data::new(dict_generic);

	HttpServer::new(move || {
		App::new()
		
		// Add the logging wrapper
		.wrap(Logger::default())
		
		// Shared data objects
		.app_data(shared_pg.clone())
		.app_data(shared_redis.clone())
		.app_data(shared_metadata.clone())

		// Simple paths must be listed first, to resolve earlier
		.service(route_refresh::refresh)
		.service(route_create::create)
		.service(route_namefree::namefree)
		//.service(route_agecheck::agecheck)

		// Wildcard paths MUST be below fixed paths, otherwise
		// "GET /refresh" resolves to "GET /{provider=refresh}"
		// TODO: These need a separate path/folder to make sure they don't collide with other routes
		.service(route_provider::provider)
		.service(route_callback::callback)
		
		// Healthcheck routes
		.service(health_check::livez_status)
		.service(health_check::readyz_status)
	})

	// All Imgcat services bind to :8080, for predictability
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
