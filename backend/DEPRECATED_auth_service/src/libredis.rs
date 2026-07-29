use std::sync::Mutex;
use redis::{Client, Cmd, Connection, RedisError};
use std::time::Duration;


const CONN_TIMEOUT:Duration = Duration::from_secs(3);

// Just a dumb storage struct
pub struct LoginState {
	pub state: String,
	pub nonce: String,
	pub pkce_v: String,
	pub redirect: Option<String>,
}

// use a mutex so we can use a single pool for each thread
pub struct AppStateRedis {
	pool: Mutex<Client>
}

impl AppStateRedis {
	pub fn new() -> Self {
		// TODO - messy, use ConnectionInfo
		Self {
			pool: Mutex::new(
				Client::open(format!(
					"redis://{}:{}/{}",
					std::env::var("IC_REDIS_HOST").expect("Could not parse envvar: IC_REDIS_HOST"),
					std::env::var("IC_REDIS_PORT").unwrap_or("8080".to_string()),
					std::env::var("IC_REDIS_DB").unwrap_or("".to_string()),
				)).expect("Connection to Redis failed. Check envvars IC_REDIS_HOST/PORT/DB."))
		}
	}

	fn get_conn(&self) -> Result<Connection, RedisError> {
		// TODO: If the pool is poisoned, try to recover w/o a panic
		let client = self.pool.lock().expect("Poisoned Redis pool");
		return client.get_connection_with_timeout(CONN_TIMEOUT);
	}

	pub fn set_login(&self, data: &LoginState) -> Result<(), RedisError> {
		let mut conn = self.get_conn()?;
		let key = format!("login:{}", data.state);
		let value = format!("{};{};{}",
			data.nonce,
			data.pkce_v,
			data.redirect.as_deref().unwrap_or(""));

		Cmd::new()
			.arg("SET").arg(key).arg(value)
			.arg("EX").arg("300") // A login attempt expires after 5 mins
			.exec(&mut conn)?;
		return Ok(());
	}

	pub fn get_login(&self, state: &String) -> Result<LoginState, RedisError> {
		let mut conn = self.get_conn()?;
		let value = Cmd::new()
			.arg("GETDEL") // Clean up & make sure it can't be reused
			.arg(format!("login:{}", state))
			.query::<String>(&mut conn)?;

		// Format: {nonce};{pkce_verifier};Option<{redirect_url}>
		let value:Vec<&str> = value.splitn(3, ';').collect();
		if value.len() != 3 {
			panic!("Redis validation error");
		}

		return Ok(LoginState {
			state: state.to_string(),
			nonce: value[0].to_string(),
			pkce_v: value[1].to_string(),
			redirect: if value[2]!="" { Some(value[2].to_string())} else { None },
		})
	}

	pub fn is_user_banned(&self, account_id:i64) -> Result<bool, RedisError> {
		let mut conn = self.get_conn()?;
		let value:i64 = Cmd::new()
			.arg("EXISTS")
			.arg(format!("ban:{}", account_id))
			.query(&mut conn)?;
		return Ok(value > 0);
	}
}