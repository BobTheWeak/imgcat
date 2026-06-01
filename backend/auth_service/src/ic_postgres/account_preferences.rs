use serde::{Serialize, Deserialize};
use tokio_postgres::{Client, Row, Error, types::Type};
use crate::ic_postgres::{ ContentLevel, VisibilityLevel };


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPreferences {
	pub account_id: i64,
	pub username: String,
	pub about_me: String,

	content_level: (ContentLevel, ContentLevel),
	see_sexuality: (bool, bool),
	see_gore: (bool, bool),
	see_trauma: (bool, bool),

	news_weight: i16,
	politics_weight: i16,
	creators_weight: i16,
	selfies_weight: i16,
	pets_weight: i16,
	ai_weight: i16,

	about_me_visibility: (VisibilityLevel, VisibilityLevel),
	activity_visibility: (VisibilityLevel, VisibilityLevel),
	dm_visibility: (VisibilityLevel, VisibilityLevel),
}


pub async fn get_account_preferences(client:&Client, account_id:i64) -> Result<Option<AccountPreferences>, Error> {

	// Returns (0-1 rows): lots of fields
	let row:Option<Row> = client.query_typed_opt(
		"SELECT * FROM UserDB.GetAccountPreferences($1)", &[
		(&account_id, Type::INT8), //BIGINT
	]).await?;

	if let Some(row) = row {
		Ok(Some(AccountPreferences {
			account_id: row.get("account_id"),
			username:   row.get("username"),
			about_me:   row.get("about_me"),

			content_level: (
				row.get("content_level"),
				row.get("legal_content_level"),
			),
			see_sexuality: (
				row.get("see_sexuality"),
				row.get("legal_see_sexuality"),
			),
			see_gore:      (
				row.get("see_gore"),
				row.get("legal_see_gore"),
			),
			see_trauma:    (
				row.get("see_trauma"),
				row.get("legal_see_trauma"),
			),

			news_weight:     row.get("news_weight"),
			politics_weight: row.get("politics_weight"),
			creators_weight: row.get("creators_weight"),
			selfies_weight:  row.get("selfies_weight"),
			pets_weight:     row.get("pets_weight"),
			ai_weight:       row.get("ai_weight"),

			about_me_visibility: (
				row.get("about_me_visibility"),
				row.get("legal_about_me_visibility"),
			),
			activity_visibility: (
				row.get("activity_visibility"),
				row.get("legal_activity_visibility"),
			),
			dm_visibility:       (
				row.get("dm_visibility"),
				row.get("legal_dm_visibility"),
			),
		}))
	} else {
		Ok(None)
	}
}
