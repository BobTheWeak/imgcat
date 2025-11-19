use std::sync::OnceLock;
use std::time::Duration;
use sqlx::Error;
use crate::{IC_DB_HOST, IC_DB_USER, IC_DB_PASS};

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
pub async fn connect() -> Result<&'static MySqlPool, Error> {
	if let Some(p) = POOL.get() {
		return Ok(p);
	} else {
		let conn = MySqlConnectOptions::new()
			.host(IC_DB_HOST)
			.username(IC_DB_USER)
			.password(IC_DB_PASS);
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
		return POOL.get().ok_or(Error::PoolClosed);
	}
}

#[cfg(not(feature = "use_mariadb"))]
pub async fn connect() -> Result<&'static PgPool, Error> {
	if let Some(p) = POOL.get() {
		return Ok(p);
	} else {
		let conn = PgConnectOptions::new()
			.host(IC_DB_HOST)
			.username(IC_DB_USER)
			.password(IC_DB_PASS);
		let pool = PgPoolOptions::new()
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
		return POOL.get().ok_or(Error::PoolClosed);
	}
}