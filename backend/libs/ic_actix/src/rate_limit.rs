use redis::Connection;

use crate::ic_error::{ICError, ICResult};


pub fn check_rate_limit(user_id_or_ip:&str, conn:&mut Connection) -> ICResult<()> {
	let Ok((result, _ttl)):Result<(i64, i64), _> = redis::cmd("FCALL")
		.arg("rlf") // Rate-limit fixed-window
		.arg("1")
		.arg(user_id_or_ip)
		.arg(120) // 120 allowed actions
		.arg(60) // in a window of 60 seconds
		.query(conn) else {
			return Err(ICError::REDIS_CONN);
	};

	if result == 1 {
		return Ok(());
	} else {
		return Err(ICError::RATE_LIMIT);
	}
}