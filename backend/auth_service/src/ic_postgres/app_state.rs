use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;
use tokio_postgres::{Row, Error, NoTls, types::Type};
use deadpool_postgres::{Config, Pool, PoolError, Client, Runtime};

use crate::age_verification::AgeVerification;
use crate::ic_postgres::{AccountData, AccountPreferences};
use crate::ic_postgres::{get_account_data as fn_get_account_data};
use crate::ic_postgres::{get_account_preferences as fn_get_account_preferences};

// use a mutex so we can use a single pool for each thread
pub struct AppStatePostgres {
	pool: Mutex<Pool>
}


impl AppStatePostgres {
	pub async fn new() -> Self {
		let mut cfg = Config::new();

		cfg.host = Some(std::env::var("IC_UDB_HOST")
			.expect("Could not parse envvar: IC_UDB_HOST"));
		cfg.port = Some(u16::from_str(
			&std::env::var("IC_UDB_PORT")
			.expect("Could not parse envvar: IC_UDB_PORT"))
			.expect("Could not parse envvar: IC_UDB_PORT"));
		cfg.dbname = Some(std::env::var("IC_UDB_DB")
			.expect("Could not parse envvar: IC_UDB_DB"));
		cfg.user = Some(std::env::var("IC_UDB_USER")
			.expect("Could not parse envvar: IC_UDB_USER"));
		cfg.password = Some(std::env::var("IC_UDB_PASS")
			.expect("Could not parse envvar: IC_UDB_PASS"));

		// Timeout params
		cfg.connect_timeout = Some(Duration::from_secs(3));

		let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)
			.expect("Could not create Postgres pool");

		// Test the pool before stashing it away
		// This will fail on startup if there's a connection issue
		let client = pool.get().await.expect("PostgreSQL connection issue");
		let _ = client.query("SELECT 1", &[]).await.expect("PostgreSQL query issue");

		Self {pool:Mutex::new(pool)}
	}

	async fn get_conn(&self) -> Result<Client, PoolError> {
		// TODO: If the pool is poisoned, try to recover w/o a panic
		let pool = self.pool.lock().expect("Poisoned Postgres mutex");
		// TODO: If we can't grab a connection, try to recover
		return pool.get().await;
	}

	// TODO: Make it smarter
	pub async fn check_pool_health(&self) -> bool {
		if let Ok(client) = self.get_conn().await {
			if let Ok(_) = client.query("SELECT 1", &[]).await {
				return true;
			}
		}
		false
	}

	pub async fn is_age_needed_on_signup(&self, country:&str, state:Option<&str>) -> Result<bool, Error> {
		let client = self.get_conn().await.expect("Postgres connection error");

		// Returns (1 row): bool
		let row:Row = client.query_typed_one(
			"SELECT Legal.IsAgeNeededOnSignup($1, $2)", &[
				(&country, Type::TEXT),
				(&state, Type::TEXT),
			])
			.await?;
		return row.try_get(0);
	}

	pub async fn get_account_id(&self, provider:&str, subject:&str) -> Result<Option<i64>, Error> {
		let client = self.get_conn().await.expect("Postgres connection error");

		// Returns (1 row): bigint|null
		let row:Row = client.query_typed_one(
				"SELECT UserDB.GetAccountId($1, $2)", &[
					(&provider, Type::TEXT),
					(&subject, Type::TEXT),
				])
				.await?;

		return row.try_get(0);
	}

	pub async fn get_account_data(&self, account_id:i64) -> Result<Option<AccountData>, Error> {
		let mut client = self.get_conn().await.expect("Postgres connection error");
		fn_get_account_data(&mut client, account_id).await
	}

	pub async fn create_account(&self, prv:&str, sub:&str, username:&str, age_ver:&Option<AgeVerification>) -> Result<Option<i64>, Error> {
		let client = self.get_conn().await.expect("Postgres connection error");

		// Returns (1 row): bigint|null
		let row:Row = client.query_typed_one(
			"SELECT UserDB.CreateAccount($1,$2,$3,$4,$5,$6)", &[
			(&prv, Type::TEXT),
			(&sub, Type::TEXT),
			(&username, Type::TEXT),
			(&age_ver.as_ref().and_then(|x| Some(x.country.clone())), Type::TEXT), // CHAR(2)
			(&age_ver.as_ref().and_then(|x| Some(x.state.clone())), Type::TEXT), // CHAR(2)
			(&age_ver.as_ref().and_then(|x| Some(x.age as i16)), Type::INT2), // SMALLINT
		]).await?;

		return row.try_get(0);
	}

	pub async fn is_username_free(&self, username:&str) -> Result<bool, Error> {
		let client = self.get_conn().await.expect("Postgres connection error");

		// Returns (1 row): bool
		let row:Row = client.query_typed_one(
			"SELECT UserDB.IsUsernameFree($1)", &[
			(&username, Type::TEXT),
		]).await?;

		return row.try_get(0);
	}

	pub async fn get_account_preferences(&self, account_id:i64) -> Result<Option<AccountPreferences>, Error> {
		let client = self.get_conn().await.expect("Postgres connection error");
		fn_get_account_preferences(&client, account_id).await
	}
}