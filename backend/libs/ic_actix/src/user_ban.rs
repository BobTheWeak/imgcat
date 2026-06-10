use redis::Cmd;

use crate::ic_error::{ICError, ICResult};
use crate::app_state_redis::AppStateRedis;
use crate::app_state_postgres::AppStatePostgres;

pub async fn check_ban(account_id:i64, redis:&AppStateRedis, pg:&AppStatePostgres) -> ICResult<()> {
	// Check if they're banned in Redis (temporarially)
	let Ok(mut conn) = redis.get_conn() else {
		return Err(ICError::REDIS_CONN);
	};

	// Check in Redis, which stores temporary bans
	let Ok(value) = Cmd::new()
		.arg("EXISTS")
		.arg(format!("ban:{}", account_id))
		.query::<usize>(&mut conn) else { return Err(ICError::REDIS_CONN) };

	if value > 0 {
		return Err(ICError::BAN_TEMP);
	}

	// Check in Postgres, which stores permanent bans
	// TODO: everything
	let _ = pg;

	// Ok, we're good!
	return Ok(());
}