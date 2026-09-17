use serde::Deserialize;
use actix_web::{HttpRequest};
use actix_web::web::{Path, Query, Data};

//use crate::DB;
//use crate::deserialize_helpers::bool_from_int;
use ic_actix::{ICResult, ICError, AppStatePostgres, AppStateRedis, get_bearer_jwt, check_temp_ban};
//use crate::conn_helpers::connect;
use ic_jwt::{AuthJwt, DecodeJwt};
use ic_datamodel::bool_as_01;
use postgres_types::Type;


#[derive(Deserialize)]
pub struct VoteCategoryParams {
	#[serde(rename="a",with="bool_as_01",default)]
	is_animals: bool,
	#[serde(rename="c",with="bool_as_01",default)]
	is_creator: bool,
	#[serde(rename="s",with="bool_as_01",default)]
	is_selfie: bool,
	#[serde(rename="n",with="bool_as_01",default)]
	is_news: bool,
	#[serde(rename="p",with="bool_as_01",default)]
	is_politics: bool,
	#[serde(rename="r",with="bool_as_01",default)]
	is_ai: bool,
}

/// A registered user believes this post is one of several divisive categories
//#[post("/vote_category/{post_id}")] // Plus optional params: p=1, n=1, etc.
pub async fn vote_category(
		path: Path<(i64,)>,
		params: Query<VoteCategoryParams>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<()> {

	// TODO: Rate-limiting

	// Grab needed data from path & query params
	let post_id:i64 = path.0;
	
	// Grab the Bearer header & check it's encoding
	let jwt_string = get_bearer_jwt(&request)?;
	// Decode the JWT & make sure it's ours
	let Ok(ajwt) = AuthJwt::decode_with_defaults(jwt_string) else {
		return Err(ICError::HEADER_VALIDATION);
	};

	// Check if the user is banned (via Redis)
	check_temp_ban(ajwt.sub, &mut redis.get_conn()?).await?;

	// Get the DB connection
	let conn = postgres.get_conn().await?;
	// TODO - This doesn't work yet
	//check_perm_ban(ajwt.sub, &conn).await?;

	match conn.query_typed_one(
		"SELECT SoftMod.SetCategoryVote(post_id:=$1, user_id:=$2, is_news:=$3, is_politics:=$4, is_oc_art:=$5, is_selfie:=$6, is_animal:=$7, is_ai:=$8)",
		&[
			(&post_id,  Type::INT8),
			(&ajwt.sub, Type::INT8),
			(&params.is_news,     Type::BOOL),
			(&params.is_politics, Type::BOOL),
			(&params.is_creator,  Type::BOOL),
			(&params.is_selfie,   Type::BOOL),
			(&params.is_animals,  Type::BOOL),
			(&params.is_ai,       Type::BOOL),
		]
	).await {
		Ok(data) => {
			// The function returns BOOL
			let result:bool = data.get(0);
			if result {
				return Ok(());
			} else {
				println!("SoftMod.SetCategoryVote returned FALSE. This should never happen.");
				return Err(ICError::error(""));
			}			
		},
		Err(e) => {
			println!("{}", e);
			return Err(ICError::POSTGRES_CONN);
		}
	};
}



