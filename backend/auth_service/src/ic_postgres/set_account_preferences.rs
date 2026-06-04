use serde::{Serialize, Deserialize};
use tokio_postgres::{Client};
use tokio_postgres::types::{ToSql, Type};
use crate::ic_postgres::{ ContentLevel, VisibilityLevel, ContentWeight };
use crate::ic_error::{ICResult, ICError};


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPreferencesSetter {
	pub account_id: i64,

	pub username: Option<String>,
	pub about_me: Option<String>,

	pub content_level: Option<ContentLevel>,
	pub see_sexuality: Option<bool>,
	pub see_gore: Option<bool>,
	pub see_trauma: Option<bool>,

	pub news_weight: Option<ContentWeight>,
	pub politics_weight: Option<ContentWeight>,
	pub creators_weight: Option<ContentWeight>,
	pub selfies_weight: Option<ContentWeight>,
	pub pets_weight: Option<ContentWeight>,
	pub ai_weight: Option<ContentWeight>,

	pub about_me_visibility: Option<VisibilityLevel>,
	pub activity_visibility: Option<VisibilityLevel>,
	pub dm_visibility: Option<VisibilityLevel>,
}


pub async fn set_account_preferences_postgres(client:&Client, prefs:&AccountPreferencesSetter) -> ICResult<i32> {

	// NOTE: This is a fucking mess. I mean, feel free to figure out a better way to implement this...

	// There are three things intersecting here: A) we need optional parameters
	// but Rust has opinions, B) we only use the query_typed variants of queries,
	// and C) custom enum types aren't aren't compatible with compiled types.
	// Therefore, we're hardcoding Enums as &'const "BLAH" directly into the query.
	// This is a SQL-injection red-flag. But it's fine. Nothing's even dynamic.

	let mut idx:u32 = 1;
	let mut query:String = "SELECT UserDB.SetAccountPreferences(account_id:=$1".to_string();
	let mut params:Vec<(&(dyn ToSql + Sync), Type)> = Vec::new();
	params.push((&prefs.account_id, Type::INT8));

	// Order doesn't matter. Any standard SQL types need to be parameterized & hardcoded ENUM types need to be stringified.
	if let Some(v)=&prefs.username            { idx+=1; query.push_str(&format!(            ",username:=${}",  idx)); params.push((v,Type::TEXT)); }
	if let Some(v)=&prefs.about_me            { idx+=1; query.push_str(&format!(            ",about_me:=${}",  idx)); params.push((v,Type::TEXT)); }
	if let Some(v)=&prefs.content_level       {         query.push_str(&format!(       ",content_level:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.see_sexuality       { idx+=1; query.push_str(&format!(       ",see_sexuality:=${}",  idx)); params.push((v,Type::BOOL)); }
	if let Some(v)=&prefs.see_gore            { idx+=1; query.push_str(&format!(            ",see_gore:=${}",  idx)); params.push((v,Type::BOOL)); }
	if let Some(v)=&prefs.see_trauma          { idx+=1; query.push_str(&format!(          ",see_trauma:=${}",  idx)); params.push((v,Type::BOOL)); }
	if let Some(v)=&prefs.news_weight         {         query.push_str(&format!(         ",news_weight:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.politics_weight     {         query.push_str(&format!(     ",politics_weight:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.creators_weight     {         query.push_str(&format!(     ",creators_weight:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.selfies_weight      {         query.push_str(&format!(      ",selfies_weight:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.pets_weight         {         query.push_str(&format!(         ",pets_weight:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.ai_weight           {         query.push_str(&format!(           ",ai_weight:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.about_me_visibility {         query.push_str(&format!( ",about_me_visibility:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.activity_visibility {         query.push_str(&format!( ",activity_visibility:='{}'", v.to_sql_text())); }
	if let Some(v)=&prefs.dm_visibility       {         query.push_str(&format!(       ",dm_visibility:='{}'", v.to_sql_text())); }

	query.push(')');

	// Returns (1 row): INT
	let Ok(row) = client.query_typed_one(&query, &params)
		.await else { return Err(ICError::POSTGRES_ERROR) };

	return Ok(row.get(0));
}
