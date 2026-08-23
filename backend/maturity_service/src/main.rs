//mod redis_helpers;

mod routes;

use std::str::FromStr;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::{Data, get};
use env_logger::Env;

use ic_actix::{AppStateRedis, AppStatePostgres};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(Env::default().default_filter_or("debug"));

	// Check for required ENVVARs
	let mut check = true;
	let required_envvars = vec![
		"IC_JWT_ISS",  // Default: Use IC_ORIGIN envvar instead
		"IC_JWT_AUD",  // Default: Use IC_ORIGIN envvar instead
		"IC_JWT_PUB",
		"IC_REDIS_HOST",
		// Some of UDB vars could be defaulted
		"IC_ACTIONS_HOST",
		"IC_ACTIONS_PORT",
		"IC_ACTIONS_DB",
		"IC_MATURITY_SVC_USER",
		"IC_MATURITY_SVC_PASS",
	];
	let optional_envvars = vec![
		"IC_JWT_PUB_ROTATED", // Default: None
		"IC_REDIS_PORT", // Default: 8080
		"IC_REDIS_DB",   // Default: None
	];
	for s in required_envvars {if std::env::var(s).is_err(){println!("Error: missing envvar {}",s);check=false}}
	for s in optional_envvars {if std::env::var(s).is_err(){println!("Info: missing envvar {}, using default",s)}}
	if !check {return Ok(())}

	// B/c ownership, spawn a connection before we stuff this into a Data object.
	// Actix does some automagic unwrapping stuff, and I don't have time to figure out how to pass & unwrap a raw, unmagic'ed Data object
	let app_state_pg = AppStatePostgres::new(
		&std::env::var("IC_ACTIONS_HOST").expect("Could not parse envvar: IC_ACTIONS_HOST"),
		u16::from_str(
			&std::env::var("IC_ACTIONS_PORT").unwrap_or("8080".to_string())
		).expect("Could not parse envvar: IC_ACTIONS_PORT"),
		&std::env::var("IC_ACTIONS_DB").expect("Could not parse envvar: IC_ACTIONS_DB"),
		&std::env::var("IC_MATURITY_SVC_USER").expect("Could not parse envvar: IC_MATURITY_SVC_USER"),
		&std::env::var("IC_MATURITY_SVC_PASS").expect("Could not parse envvar: IC_MATURITY_SVC_PASS"),
	).await.expect("Could not connect to Postgres");
	
	// Clone refs for the thread (will be moved)
	// TODO: Fetch a new conn inside the thread. Otherwise a single fail will break the service.
	let Ok(conn_thread) = app_state_pg.get_conn().await else {
		// TODO: We don't need to hard fail like this. Try again? If its still stuck after many tries, then hard fail & notify someone.
		panic!("ERROR: Could not fetch connection, halting worker thread");
	};

	// Shared state objects across the entire service pool
	let shared_worker = Data::new(tokio::sync::RwLock::new(true));
	let shared_redis = Data::new(AppStateRedis::new_with_defaults().expect("Could not connect to Redis"));
	let shared_postgres = Data::new(app_state_pg);
	//let shared_mariadb = Data::new(AppStateMariaDB::new_with_defaults().expect("Could not connect to MariaDB"));

	// Clone refs for the thread (will be moved)
	let keep_looping_thread = shared_worker.clone();

	// Spawn a single worker thread
	// TODO: This is probably a terrible way of doing this
	tokio::spawn(async move {
		// Start the timer at 0.5s sleep, but it'll move around
		let mut sleep_duration_ms:u64 = 500;
		while *keep_looping_thread.read().await {

			//let Ok(conn) = app_state.get_conn().await else {
			//	// TODO: We don't need to hard fail like this. Try again? If its still stuck after many tries, then hard fail & notify someone.
			//	panic!("ERROR: Could not fetch connection, halting worker thread");
			//};

			let posts_updated:i32 = match conn_thread.query_typed_one(
				"SELECT Results.CalculateMaturityScore()",
				&[ // We don't need to override any default params
					// p[0] => post_ids:bigint[] (default: NULL - Check the internal queue for work)
					// p[1] => batch_size:int (default: 50)
				],
			).await {
				Ok(row) => row.get(0),
				Err(_) => panic!("Error: Could not run CalculateMaturityScore()")
			};

			if posts_updated > 0 {
				// If we did work, reset the sleep timer to its lowest value & get right back to work
				sleep_duration_ms = 500;
				println!("Calculated {} posts", posts_updated);
			} else {
				// increase sleep timer by 20%, up to 30s max, then sleep
				sleep_duration_ms =  ((sleep_duration_ms as f32 * 1.2) as u64).min(30_000);
				println!("No work found, sleeping for {}ms", sleep_duration_ms);
				tokio::time::sleep(tokio::time::Duration::from_millis(sleep_duration_ms)).await;
			}

		}
	});


	HttpServer::new(move || {
		App::new()
		
		// Add the logging wrapper
		.wrap(Logger::new("%r %s %b %D"))
		
		// Shared data objects
		.app_data(shared_worker.clone())
		.app_data(shared_redis.clone())
		.app_data(shared_postgres.clone())
		//.app_data(shared_mariadb.clone())


		// TODO: I don't know what we need here...
		// * Start/stop
		// * Check queue depth
		// TODO: This probably only needs a single REST worker
		// HttpServer.workers(1)

		
		// Healthcheck routes
		.route("/livez", get().to(routes::livez_status))
		.route("/readyz", get().to(routes::readyz_status))
	})

	// All Imgcat services bind to :8080, for predictability
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
