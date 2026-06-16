use redis::{Connection, Cmd};
use deadpool_postgres::Client;

use crate::ic_error::{ICError, ICResult};
//use crate::app_state_redis::AppStateRedis;
//use crate::app_state_postgres::AppStatePostgres;


pub async fn check_temp_ban(account_id:i64, redis:&mut Connection) -> ICResult<()> {
	let Ok(value) = Cmd::new()
		.arg("EXISTS")
		.arg(format!("ban:{}", account_id))
		.query::<usize>(redis) else { return Err(ICError::REDIS_CONN) };

	if value > 0 {
		return Err(ICError::BAN_TEMP);
	}

	// Ok, we're good!
	return Ok(());
}


pub async fn check_perm_ban(account_id:i64, pg:&Client) -> ICResult<()> {
	// Check in Postgres, which stores permanent bans
	// TODO: everything

	// Ok, we're good!
	return Ok(());
}