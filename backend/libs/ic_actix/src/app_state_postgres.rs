use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;
use actix_web::web::{Data};
use deadpool_postgres::{Config, Pool, Client, Runtime};
use deadpool_postgres::tokio_postgres::{NoTls};

use crate::ic_error::{ICError, ICResult};

const CONN_TIMEOUT:Duration = Duration::from_secs(3);

pub type AppStatePostgres = Data<ICPostgresWrapper>;

// TODO: Check if a Mutex is necessary. It could be tokio/thread-safe already.
pub struct ICPostgresWrapper {
	pool: Mutex<Pool>
}

impl ICPostgresWrapper {
	pub async fn new(host:&str, port:u16, db:&str, user:&str, pass:&str) -> ICResult<Self> {

		let mut cfg = Config::new();
		cfg.host = Some(host.to_string());
		cfg.port = Some(port);
		cfg.dbname = Some(db.to_string());
		cfg.user = Some(user.to_string());
		cfg.password = Some(pass.to_string());

		cfg.connect_timeout = Some(CONN_TIMEOUT);

		let Ok(pool) = cfg.create_pool(Some(Runtime::Tokio1), NoTls) else {
			return Err(ICError::POSTGRES_CONN);
		};

		// Test the connection to make sure it works
		let Ok(client) = pool.get().await else {
			return Err(ICError::POSTGRES_CONN);
		};
		if client.query("SELECT 1", &[]).await.is_err() {
			return Err(ICError::POSTGRES_CONN);
		}

		return Ok(Self {pool:Mutex::new(pool)});
	}

	#[cfg(feature="std_envvars")]
	pub async fn new_with_defaults(db:&str, user:&str, pass:&str) -> ICResult<Self> {
		let host:&str = &std::env::var("IC_UDB_HOST").expect("Could not parse envvar: IC_UDB_HOST");
		let port:u16 = u16::from_str(
			&std::env::var("IC_UDB_PORT").unwrap_or("8080".to_string())
		).expect("Could not parse envvar: IC_UDB_PORT");

		return Self::new(host, port, db, user, pass).await;
	}

	pub async fn get_conn(&self) -> ICResult<Client> {
		let Ok(pool) = self.pool.lock() else {
			// TODO: If the pool is poisoned, try to recover
			return Err(ICError::POOL_ERROR);
		};

		let Ok(client) = pool.get().await else {
			return Err(ICError::POSTGRES_CONN);
		};
		return Ok(client);
	}
}