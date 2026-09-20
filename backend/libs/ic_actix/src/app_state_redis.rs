use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;
use redis::{Client, Connection};

use crate::ic_error::{ICError, ICResult};

const CONN_TIMEOUT:Duration = Duration::from_secs(3);

// TODO: Check if a Mutex is necessary. It could be tokio/thread-safe already.
#[derive(Debug)]
pub struct AppStateRedis {
	pool: Mutex<Client>
}

impl AppStateRedis {
	pub fn new(host:&str, port:u16, db:Option<&str>) -> ICResult<Self> {
		let conn_str:String = format!("redis://{}:{}/{}", host, port, db.unwrap_or(""));
		let Ok(result) = Client::open(conn_str) else {
			return Err(ICError::REDIS_CONN);
		};

		// TODO: Test the connection
		Ok(Self {pool: Mutex::new(result)})
	}

	#[cfg(feature="std_envvars")]
	pub fn new_with_defaults() -> ICResult<Self> {
		let host:&str = &std::env::var("IC_REDIS_HOST").expect("Could not parse envvar: IC_REDIS_HOST");
		let port:u16 = u16::from_str(
			&std::env::var("IC_REDIS_PORT").unwrap_or("8080".to_string())
		).expect("Could not parse envvar: IC_REDIS_PORT");
		let db:Option<String> = std::env::var("IC_REDIS_DB").ok();

		return Self::new(host, port, db.as_deref());
	}

	pub fn get_conn(&self) -> ICResult<Connection> {
		let Ok(client) = self.pool.lock() else {
			// TODO: If the pool is poisoned, try to recover
			return Err(ICError::POOL_ERROR);
		};

		let Ok(conn) = client.get_connection_with_timeout(CONN_TIMEOUT) else {
			return Err(ICError::REDIS_CONN);
		};
		return Ok(conn);
	}

	pub fn check_rate_limit(&self, user_id_or_ip:&str) -> ICResult<()> {
		let mut conn = self.get_conn()?;
		let Ok((result, _ttl)):Result<(i64, i64), _> = redis::cmd("FCALL")
			.arg("rlf") // Rate-limit fixed-window
			.arg("1")
			.arg(user_id_or_ip)
			.arg(120) // 120 allowed actions
			.arg(60) // in a window of 60 seconds
			.query(&mut conn) else {
				return Err(ICError::REDIS_CONN);
		};

		if result == 1 {
			return Ok(());
		} else {
			return Err(ICError::RATE_LIMIT);
		}
	}
}