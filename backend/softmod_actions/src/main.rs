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

use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(Env::default().default_filter_or("info"));

	HttpServer::new(|| {
		App::new()
		.wrap(Logger::default())
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
