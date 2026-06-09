use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;
use sqlx::Error;

// We're keeping max conns pretty low, depending on multiple instances to provide
// most of the heavy lifting. With that said, every query should be instant.
const MIN_CONNS:u32 = 0;
const MAX_CONNS:u32 = 4;
// If we haven't gotten a request in 5 mins, fine, you can go to sleep
const IDLE_TIMEOUT:Duration = Duration::from_secs(60*5);
// Recycle the connection every so often (helps the DB stay clean)
const RECYCLE_TIMEOUT:Duration = Duration::from_secs(60*60);

#[cfg(feature = "use_mariadb")]
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlConnectOptions};
#[cfg(not(feature = "use_mariadb"))]
use sqlx::postgres::{PgPool, PgPoolOptions, PgConnectOptions};

#[cfg(feature = "use_mariadb")]
static POOL:OnceLock<MySqlPool> = OnceLock::new();
#[cfg(not(feature = "use_mariadb"))]
static POOL:OnceLock<PgPool> = OnceLock::new();



#[cfg(feature = "use_mariadb")]
pub async fn connect() -> Result<MySqlPool, Error> {
	if let Some(p) = POOL.get() {
		return Ok(p.clone());
	} else {
		let ic_db_host:&str = &std::env::var("IC_DB_HOST").expect("EnvVar not set: IC_DB_HOST");
		let ic_db_port:&str = &std::env::var("IC_DB_PORT").expect("EnvVar not set: IC_DB_PORT");
		// TODO: Set it up with the default
		//let ic_db_db:&str = &std::env::var("IC_DB_DB").expect("EnvVar not set: IC_DB_DB");
		let ic_db_user:&str = &std::env::var("IC_DB_USER").expect("EnvVar not set: IC_DB_USER");
		let ic_db_pass:&str = &std::env::var("IC_DB_PASS").expect("EnvVar not set: IC_DB_PASS");

		println!("Creating new pool: {}:{}", ic_db_host, ic_db_port);
		let conn = MySqlConnectOptions::new()
			.host(ic_db_host)
			.port(u16::from_str(ic_db_port).expect("IC_DB_PORT could not be parsed"))
			.username(ic_db_user)
			.password(ic_db_pass);
		let pool = MySqlPoolOptions::new()
			.min_connections(MIN_CONNS)
			.max_connections(MAX_CONNS)
			.idle_timeout(IDLE_TIMEOUT)
			.max_lifetime(RECYCLE_TIMEOUT)
			.connect_with(conn).await;
		if let Err(p) = POOL.set(pool?) {
			// If this errors, we probably hit a race condition
			// But no problem... just cleanup our pool.
			p.close().await;
		}

		if let Some(p) = POOL.get() {
			return Ok(p.clone());
		} else {
			return Err(Error::PoolClosed);
		}
	}
}

//#[cfg(not(feature = "use_mariadb"))]
//pub async fn connect() -> Result<&'static PgPool, Error> {
//	if let Some(p) = POOL.get() {
//		return Ok(p);
//	} else {
//		let conn = PgConnectOptions::new()
//			.host(IC_DB_HOST)
//			.port(u16::from_str(IC_DB_PORT).expect("IC_DB_PORT could not be parsed"))
//			.username(IC_DB_USER)
//			.password(IC_DB_PASS);
//		let pool = PgPoolOptions::new()
//			.min_connections(MIN_CONNS)
//			.max_connections(MAX_CONNS)
//			.idle_timeout(IDLE_TIMEOUT)
//			.max_lifetime(RECYCLE_TIMEOUT)
//			.connect_with(conn).await;
//		if let Err(p) = POOL.set(pool?) {
//			// If this errors, we probably hit a race condition
//			// But no problem... just cleanup our pool.
//			p.close().await;
//		}
//		return POOL.get().ok_or(Error::PoolClosed);
//	}
//}