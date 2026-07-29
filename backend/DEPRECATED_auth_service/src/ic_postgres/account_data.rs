use serde::{Serialize, Deserialize};
use tokio_postgres::{Client, Row, Error, types::Type};


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
	pub account_id: i64,
	pub username: String,
	pub claims: Vec<String>,
}


pub async fn get_account_data(client:&mut Client, account_id:i64) -> Result<Option<AccountData>, Error> {
	let transaction = client.transaction().await.expect("Postgres connection error");

	// Returns (1 row): i64|null, text|null, test|null
	let row:Row = transaction.query_typed_one(
		// BUG: The postgres_types library DOES NOT have a FromSQL defined for a RefCursor type
		// This means we have to cast it as a string in SQL, to be able to read it in Rust
		"SELECT id, username, claims_cur::TEXT FROM UserDB.GetAccountData($1)", &[
		(&account_id, Type::INT8), //BIGINT
	]).await?;

	if let Some(account_id) = row.get::<_,Option<i64>>("id") {
		let username = row.get("username");
		let claims_cur:String = row.get("claims_cur");

		// Returns (N rows): text
		let rows:Vec<Row> = transaction.query_typed(
			&format!("FETCH ALL IN \"{}\"", &claims_cur), &[]
		).await?;

		transaction.commit().await?;

		Ok(Some(AccountData {
			account_id,
			username,
			claims: rows.into_iter().map(|r|{r.get::<_,String>(0)}).collect(),
		}))
	} else {
		transaction.commit().await?;
		Ok(None)
	}
}