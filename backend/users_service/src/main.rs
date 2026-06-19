mod db_objects;

mod redis_helpers;

mod routes;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::{Data, get, post, scope};
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
	//let shared_mariadb = Data::new(AppStateMariaDB::new_with_defaults().expect("Could not connect to MariaDB"));

	HttpServer::new(move || {
		App::new()
		
		// Add the logging wrapper
		.wrap(Logger::new("%r %s %b %D"))
		
		// Shared data objects
		.app_data(shared_redis.clone())
		.app_data(shared_postgres.clone())
		//.app_data(shared_mariadb.clone())


		// /my
		//    /prefs     GET  AuthJwt   Get your account & prefs
		//               POST AuthJwt   Edit your account & prefs
		//    /profile   GET  AuthJwt   Get your profile
		//    /posts     GET  AuthJwt   Get your list of posts
		//    /favs      GET  AuthJwt   Get your list of favorites
		//    /comments  GET  AuthJwt   Get your list of comments
		// /u/{account_link}
		//    /profile   GET  Public   Get a user's public profile
		//    /posts     GET  Public   Get a user's public posts
		//    /favs      GET  Public   Get a user's public favorites
		//    /comments  GET  Public   Get a user's public comments
		// /ub           POST Public   Get list of user badges
		//               Redis: Read/write "ub:"

		.service(scope("/my")
			//.route("/prevs", get().to(routes::get_my_prefs))
			//.route("/prevs", post().to(routes::set_my_prefs))
			//.route("/favs", get().to(routes::get_my_favs))
			//.route("/comments", get().to(routes::get_my_comments))
		)
		// TODO: Need a guard on {post_link} to make sure it's len=12 and Base64Url-encoded
		.service(scope("/u/{post_link}")
			//.route("/profile", get().to(routes::get_user_profile))
			//.route("/posts", get().to(routes::get_user_posts))
			//.route("/favs", get().to(routes::get_user_favs))
			//.route("/comments", get().to(routes::get_user_comments))
		)
		.route("/ub", post().to(routes::get_user_badges))

		
		// Healthcheck routes
		.route("/livez", get().to(routes::livez_status))
		.route("/readyz", get().to(routes::readyz_status))
	})

	// All Imgcat services bind to :8080, for predictability
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
