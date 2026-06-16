use std::str::FromStr;
use std::time::Duration;
use mysql::{OptsBuilder, Pool, PoolOpts, PoolConstraints, PooledConn};
use mysql::prelude::Queryable;

use crate::ic_error::{ICError, ICResult};

const CONN_TIMEOUT:Duration = Duration::from_secs(3);

#[derive(Debug)]
pub struct AppStateMariaDB {
	pool: Pool // Thread-safe, so we don't need a mutex
}

impl AppStateMariaDB {
	pub fn new(host:&str, port:u16, db:&str, user:&str, pass:&str) -> ICResult<Self> {

		let cfg = OptsBuilder::new()
			.ip_or_hostname(Some(host.to_string()))
			.tcp_port(port)
			.db_name(Some(db.to_string()))
			.user(Some(user.to_string()))
			.pass(Some(pass.to_string()))

			// Aggressive timeouts
			//.tcp_connect_timeout(Some(CONN_TIMEOUT))
			//.read_timeout(Some(CONN_TIMEOUT))
			//.write_timeout(Some(CONN_TIMEOUT))

			// defaults
			// Don't prepare any statements, we're only calling Functions
			.stmt_cache_size(Some(0))
			// The server isn't local, so don't attempt a socket/pipe conn
			.prefer_socket(false)

			// Pool-specific options
			.pool_opts(Some(
				PoolOpts::new()
				// NOTE: The default setting is 10-100 connections, which seems insane
				.with_constraints(PoolConstraints::new(1, 10).unwrap())
				//.with_reset_connection(false) // Normally risky, but calling FUNCs doesn't need resets
				//.with_check_health(false) // Lets not baby our system
			));

		let Ok(pool) = Pool::new(cfg) else {
			return Err(ICError::MARIADB_CONN);
		};

		// Test the connection to make sure it works
		let Ok(mut client) = pool.get_conn() else {
			return Err(ICError::MARIADB_CONN);
		};
		if client.exec_drop("SELECT 1", ()).is_err() {
			return Err(ICError::MARIADB_CONN);
		};

		return Ok(Self { pool:pool });
	}

	#[cfg(feature="std_envvars")]
	pub fn new_with_defaults() -> ICResult<Self> {
		let host:&str = &std::env::var("IC_DB_HOST").expect("Could not parse envvar: IC_DB_HOST");
		let port:u16 = u16::from_str(
			&std::env::var("IC_DB_PORT").unwrap_or("8080".to_string())
		).expect("Could not parse envvar: IC_DB_PORT");
		let db:&str = &std::env::var("IC_DB_DB").expect("Could not parse envvar: IC_DB_DB");
		let user:&str = &std::env::var("IC_DB_USER").expect("Could not parse envvar: IC_DB_USER");
		let pass:&str = &std::env::var("IC_DB_PASS").expect("Could not parse envvar: IC_DB_PASS");

		return Self::new(host, port, db, user, pass);
	}

	pub fn get_conn(&self) -> ICResult<PooledConn> {
		// NOTE: We don't need a mutex
		//let Ok(pool) = self.pool.clone() else {
		//	// TODO: If the pool is poisoned, try to recover
		//	return Err(ICError::POOL_ERROR);
		//};

		let Ok(client) = self.pool.get_conn() else {
			return Err(ICError::MARIADB_CONN);
		};
		return Ok(client);
	}
}