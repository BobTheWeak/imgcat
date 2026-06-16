mod db_objects;

mod redis_helpers;

mod routes;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::{Data, get, scope};
use env_logger::Env;

use ic_actix::{AppStateRedis, AppStatePostgres, AppStateMariaDB};

//use crate::ic_postgres::{AppStatePostgres};
//use crate::libredis::{AppStateRedis};
//use crate::app_state::{AppStateProviders, AppStateItem};


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
		"IC_DB_HOST",
		"IC_DB_PORT",
		"IC_DB_DB",
		"IC_DB_USER",
		"IC_DB_PASS",
		"IC_UDB_HOST",
		"IC_UDB_PORT",
		"IC_UDB_DB",
		"IC_UDB_USER",
		"IC_UDB_PASS",
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
	let shared_postgres = Data::new(AppStatePostgres::new_with_defaults().await.expect("Could not connect to Postgres"));
	let shared_mariadb = Data::new(AppStateMariaDB::new_with_defaults().expect("Could not connect to MariaDB"));

	HttpServer::new(move || {
		App::new()
		
		// Add the logging wrapper
		.wrap(Logger::new("%r %s %b %D"))
		
		// Shared data objects
		.app_data(shared_redis.clone())
		.app_data(shared_postgres.clone())
		.app_data(shared_mariadb.clone())

		//.route("/fp", get().to(routes::get_fp))
		//.route("/viral", get().to(routes::get_viral))
		//.route("/recent", get().to(routes::get_recent))

		// TODO: Need a guard on {post_link} to make sure it's len=12 and Base64Url-encoded
		.service(scope("/p/{post_link}")
			//.route("/data", get().to(routes::get_data))
			.route("/comments", get().to(routes::get_comments))
			.route("/views", get().to(routes::get_views))
			.route("/votes", get().to(routes::get_votes))
		)
		
		// Healthcheck routes
		.route("/livez", get().to(routes::livez_status))
		.route("/readyz", get().to(routes::readyz_status))
	})

	// All Imgcat services bind to :8080, for predictability
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
