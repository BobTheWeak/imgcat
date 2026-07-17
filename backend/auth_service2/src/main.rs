mod db_objects;

mod redis_helpers;
mod postgres_helpers;

mod routes;
mod common_callback;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::{Data, get, post, scope};
use env_logger::Env;

use ic_actix::{AppStateRedis, AppStatePostgres};

const SUPPORTED_PROVIDERS:[&str; 2] = ["google", "microsoft"];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(Env::default().default_filter_or("warn"));

	// Check for required ENVVARs
	let mut check = true;
	let required_envvars = vec![
		"IC_JWT_ISS",  // Default: Use IC_ORIGIN envvar instead
		"IC_JWT_AUD",  // Default: Use IC_ORIGIN envvar instead
		"IC_JWT_PUB",
		"IC_REDIS_HOST",
		// Some of UDB vars could be defaulted
		"IC_UDB_HOST",
		"IC_UDB_PORT",
		"IC_UDB_DB",
		"IC_USERS_SVC_USER",
		"IC_USERS_SVC_PASS",
		//"IC_UDB_HOST",
		//"IC_UDB_PORT",
		//"IC_UDB_DB",
		//"IC_UDB_USER",
		//"IC_UDB_PASS",
		"IC_OAUTH_GOOGLE_ID",
		"IC_OAUTH_GOOGLE_SECRET",
		"IC_OAUTH_MICROSOFT_ID",
		"IC_OAUTH_MICROSOFT_SECRET",
	];
	let optional_envvars = vec![
		"IC_JWT_PUB_ROTATED", // Default: None
		"IC_REDIS_PORT", // Default: 8080
		"IC_REDIS_DB",   // Default: None
	];
	for s in required_envvars {if std::env::var(s).is_err(){println!("Error: missing envvar {}",s);check=false}}
	for s in optional_envvars {if std::env::var(s).is_err(){println!("Info: missing envvar {}, using default",s)}}
	if !check {return Ok(())}

	// Shared state objects across the entire service pool
	let shared_redis = Data::new(AppStateRedis::new_with_defaults().expect("Could not connect to Redis"));
	let shared_postgres = Data::new(AppStatePostgres::new_with_user(
		&std::env::var("IC_USERS_SVC_USER").expect("Could not parse envvar: IC_USERS_SVC_USER"),
		&std::env::var("IC_USERS_SVC_PASS").expect("Could not parse envvar: IC_USERS_SVC_PASS"),
	).await.expect("Could not connect to Postgres"));


	// ==============================================
	// ===   Authentication Service (/api/auth)   ===
	// ==============================================
	// /p/{provider}    GET  Public       Get auth token from provider
	//                  Redis: Write "login:"
	// /c/{provider}    GET  OpenID       Auth callback from provider
	//                  Redis: Read "login:"
	// /nf/{username}   GET  AnyJWT       Is username free?
	// /r               GET  RefreshJwt   Refresh auth_jwt
	// /new             POST SignupJwt    Create a new user
	// /link            POST AuthJwt      Add another provider


	HttpServer::new(move || {
		App::new()
		
		// Add the logging wrapper (timing in ms)
		.wrap(Logger::new("%r %s %b %D"))
		
		// Shared data objects
		.app_data(shared_redis.clone())
		.app_data(shared_postgres.clone())


		.service(scope("/p")
			.route("/google", get().to(routes::provider_google))
			.route("/microsoft", get().to(routes::provider_microsoft))
		)

		.service(scope("/cb")
			.route("/google", get().to(routes::callback_google))
			.route("/microsoft", get().to(routes::callback_microsoft))
		)

		.route("/nf/{username}", get().to(routes::is_username_free_by_path))
		.route("/r", get().to(routes::refresh_jwt))
		.route("/new", post().to(routes::new_account))

		// Legacy call
		//.route("/namefree", get().to(routes::is_username_free_by_query))
		
		// Healthcheck routes
		.route("/livez", get().to(routes::livez_status))
		.route("/readyz", get().to(routes::readyz_status))
	})

	// All Imgcat services bind to :8080, for predictability
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
