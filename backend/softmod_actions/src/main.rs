// NOTE: sqlx has AnyConnection, etc. to make this easy, but there are things that
// just don't automagic convert. "conn_helpers.rs" is particularly ugly.
type DB = sqlx::MySql;

mod deserialize_helpers;
mod conn_helpers;
mod health_check;

mod vote_tag;
mod vote_mature;
mod vote_category;
mod vote_review;
mod anon_review;

use std::str::FromStr;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::{Data};
use env_logger::Env;

use ic_actix::AppStatePostgres;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(Env::default().default_filter_or("info"));

	// Check for required ENVVARs
	// TODO - This is an older microservice, so we need to review the code to do this. Then C&P the standard block we use everywhere else.

	// B/c ownership, spawn a connection before we stuff this into a Data object.
	// Actix does some automagic unwrapping stuff, and I don't have time to figure out how to pass & unwrap a raw, unmagic'ed Data object
	let app_state_pg = AppStatePostgres::new(
		&std::env::var("IC_ACTIONS_HOST").expect("Could not parse envvar: IC_ACTIONS_HOST"),
		u16::from_str(
			&std::env::var("IC_ACTIONS_PORT").unwrap_or("8080".to_string())
		).expect("Could not parse envvar: IC_ACTIONS_PORT"),
		&std::env::var("IC_ACTIONS_DB").expect("Could not parse envvar: IC_ACTIONS_DB"),
		&std::env::var("IC_MATURITY_SVC_USER").expect("Could not parse envvar: IC_SOFTMOD_SVC_USER"),
		&std::env::var("IC_MATURITY_SVC_PASS").expect("Could not parse envvar: IC_SOFTMOD_SVC_PASS"),
	).await.expect("Could not connect to Postgres");

	// Test the connection
	// TODO: It's probably smart to actually run a query too
	let Ok(_) = app_state_pg.get_conn().await else {
		// TODO: We don't need to hard fail like this. Try again? If its still stuck after many tries, then hard fail & notify someone.
		panic!("ERROR: Could not fetch connection, halting worker thread");
	};

	let app_state_pg_wrapper = Data::new(app_state_pg);

	HttpServer::new(move || {
		App::new()
		
		// Add the logging wrapper
		.wrap(Logger::new("%r %s %b %D"))

		// Shared data objects
		.app_data(app_state_pg_wrapper.clone())

		.service(vote_tag::vote_tag)
		.service(vote_mature::vote_mature)
		.service(vote_category::vote_category)
		.service(vote_review::vote_review)
		.service(anon_review::anon_review)
		
		// Healthcheck services
		.service(health_check::livez_status)
		.service(health_check::readyz_status)
	})
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
