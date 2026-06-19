use redis::{Connection, Cmd};

use ic_actix::{ICResult, ICError};
use crate::db_objects::UserBadge;

pub(crate) fn get_user_badges(conn:&mut Connection, ids:&Vec<i64>) -> ICResult<Vec<Option<UserBadge>>> {
	let mut cmd = Cmd::new();
	cmd.arg("MGET");
	for id in ids {
		cmd.arg(format!("ub:{}", id));
	}
	
	let result = cmd.query::<Vec<Option<String>>>(conn).or(Err(ICError::REDIS_CONN))?;// else { return Err(ICError::REDIS_CONN) };
	
	// Reach into the 
	return Ok(result.into_iter().map(|x| {
		if let Some(s) = x {
			Some(serde_json::from_str(&s).unwrap())
		} else {
			None
		}
	}).collect());
}

pub(crate) fn set_user_badges(conn:&mut Connection, values:&Vec<UserBadge>) -> ICResult<()> {
	let mut cmd = Cmd::new();
	cmd.arg("MSET");

	for user in values.iter() {
		let s = serde_json::to_string(user).unwrap();
		cmd.arg(format!("ub:{}", user.id));
		cmd.arg(s);
	}

	return cmd.exec(conn).or(Err(ICError::REDIS_CONN));
}
