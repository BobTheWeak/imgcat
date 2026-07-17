use serde::{Serialize, Deserialize};
use deadpool_postgres::tokio_postgres::{Client, Row, types::Type};

use ic_jwt::AgeVerification;
use ic_actix::{ICResult, ICError};


pub async fn get_account_id(client:&mut Client, provider:&str, subject:&str) -> ICResult<Option<i64>> {
	let Ok(row) = client.query_typed_one(
		"SELECT UserDB.GetAccountId($1, $2)", &[
			(&provider, Type::TEXT),
			(&subject, Type::TEXT),
		]).await else {
			return Err(ICError::POSTGRES_CONN)
	};

	let Ok(val):Result<Option<i64>, _> = row.try_get(0) else {
		return Err(ICError::POSTGRES_CONN)
	};

	return Ok(val);
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
	pub account_id: i64,
	pub username: String,
	pub claims: Vec<String>,
}

pub async fn get_account_data(client:&mut Client, account_id:i64) -> ICResult<Option<AccountData>> {
	let Ok(transaction) = client.transaction().await else {
		return Err(ICError::POSTGRES_CONN);
	};

	// Returns (1 row): i64|null, text|null, test|null
	let Ok(row):Result<Row, _> = transaction.query_typed_one(
		// BUG: The postgres_types library DOES NOT have a FromSQL defined for a RefCursor type
		// This means we have to cast it as a string in SQL, to be able to read it in Rust
		"SELECT id, username, claims_cur::TEXT FROM UserDB.GetAccountData($1)", &[
		(&account_id, Type::INT8), //BIGINT
	]).await else { return Err(ICError::POSTGRES_CONN) };

	if let Some(account_id) = row.get::<_,Option<i64>>("id") {
		let username = row.get("username");
		let claims_cur:String = row.get("claims_cur");

		// Returns (N rows): text
		let Ok(rows):Result<Vec<Row>, _> = transaction.query_typed(
			&format!("FETCH ALL IN \"{}\"", &claims_cur), &[]
		).await else { return Err(ICError::POSTGRES_CONN) };

		transaction.commit().await.or(Err(ICError::POSTGRES_CONN))?;

		Ok(Some(AccountData {
			account_id,
			username,
			claims: rows.into_iter().map(|r|{r.get::<_,String>(0)}).collect(),
		}))
	} else {
		transaction.commit().await.or(Err(ICError::POSTGRES_CONN))?;
		Ok(None)
	}
}


pub async fn is_age_needed_on_signup(conn:&Client, country:&str, state:Option<&str>) -> ICResult<bool> {

	// Returns (1 row): bool
	let Ok(row):Result<Row, _> = conn.query_typed_one(
		"SELECT Legal.IsAgeNeededOnSignup($1, $2)", &[
		(&country, Type::TEXT),
		(&state, Type::TEXT),
	]).await else {
		return Err(ICError::POSTGRES_CONN)
	};

	let Ok(value) = row.try_get::<_,bool>(0) else {
		return Err(ICError::POSTGRES_CONN)
	};

	return Ok(value);
}


pub async fn is_username_free(conn:&Client, username:&str) -> ICResult<bool> {
	//let client = self.get_conn().await.expect("Postgres connection error");

	// Returns (1 row): bool
	let Ok(row):Result<Row, _> = conn.query_typed_one(
		"SELECT UserDB.IsUsernameFree($1)", &[
		(&username, Type::TEXT),
	]).await else {
		return Err(ICError::POSTGRES_CONN)
	};

	let Ok(value) = row.try_get::<_,bool>(0) else {
		return Err(ICError::POSTGRES_CONN)
	};

	return Ok(value);
}

pub async fn create_account(conn:&Client, prv:&str, sub:&str, username:&str, age_ver:&Option<AgeVerification>) -> ICResult<Option<i64>> {

	// Returns (1 row): bigint|null
	let Ok(row):Result<Row, _> = conn.query_typed_one(
		"SELECT UserDB.CreateAccount($1,$2,$3,$4,$5,$6)", &[
		(&prv, Type::TEXT),
		(&sub, Type::TEXT),
		(&username, Type::TEXT),
		(&age_ver.as_ref().and_then(|x| Some(x.country.clone())), Type::TEXT), // CHAR(2)
		(&age_ver.as_ref().and_then(|x| Some(x.state.clone())), Type::TEXT), // CHAR(2)
		(&age_ver.as_ref().and_then(|x| Some(x.age as i16)), Type::INT2), // SMALLINT
	]).await else {
		return Err(ICError::POSTGRES_CONN)
	};

	let Ok(value) = row.try_get::<_,Option<i64>>(0) else {
		return Err(ICError::POSTGRES_CONN)
	};

	return Ok(value);
}
