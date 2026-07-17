use redis::{Connection, Cmd};
use ic_actix::{ICResult, ICError};


// Just a dumb storage struct
#[derive(Debug)]
pub struct LoginState {
	pub state: String,
	pub nonce: String,
	pub pkce_v: String,
	pub redirect: Option<String>,
}

pub fn set_login(conn:&mut Connection, data: &LoginState) -> ICResult<()> {
	let key = format!("login:{}", data.state);
	let value = format!("{};{};{}",
		data.nonce,
		data.pkce_v,
		data.redirect.as_deref().unwrap_or(""));

	let Ok(_) = Cmd::new()
		.arg("SET").arg(key).arg(value)
		.arg("EX").arg("300") // A login attempt expires after 5 mins
		.exec(conn) else { return Err(ICError::REDIS_CONN) };
	return Ok(());
}

pub fn get_login(conn:&mut Connection, state: &String) -> ICResult<LoginState> {
	let Ok(value) = Cmd::new()
		.arg("GETDEL") // Clean up & make sure it can't be reused
		.arg(format!("login:{}", state))
		.query::<String>(conn) else { return Err(ICError::REDIS_CONN) };

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

pub fn is_user_banned(conn:&mut Connection, account_id:i64) -> ICResult<bool> {
	let Ok(value):Result<i64,_> = Cmd::new()
		.arg("EXISTS")
		.arg(format!("ban:{}", account_id))
		.query(conn) else { return Err(ICError::REDIS_CONN) };
	return Ok(value > 0);
}