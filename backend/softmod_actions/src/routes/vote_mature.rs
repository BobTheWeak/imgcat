use serde::Deserialize;
use actix_web::{HttpRequest};
use actix_web::web::{Path, Query, Data};

use ic_actix::{ICResult, ICError, AppStatePostgres, AppStateRedis, get_bearer_jwt, check_temp_ban, get_user_ip};
use ic_jwt::{AuthJwt, DecodeJwt};
use ic_datamodel::bool_as_01;
use postgres_types::Type;


#[derive(Deserialize)]
pub struct VoteMatureParams {
	#[serde(rename="m")]
	maturity: u8,
	#[serde(rename="s",with="bool_as_01",default)]
	is_sexual: bool,
	#[serde(rename="g",with="bool_as_01",default)]
	is_gore: bool,
	#[serde(rename="t",with="bool_as_01",default)]
	is_trauma: bool,
}


/// A registered user believes this post is mature, spicy, etc.
pub async fn vote_maturity(
		path: Path<(i64,)>,
		params: Query<VoteMatureParams>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<()> {

	let mut conn_r = redis.get_conn()?;

	// Get user IP for rate-limiting
	if let Some(user_ip) = get_user_ip(request.headers())? {
		redis.check_rate_limit_conn(user_ip, &mut conn_r)?;
	}

	// Grab the Bearer header & decode it into an AuthJwt
	let Ok(ajwt) = AuthJwt::decode_with_defaults(get_bearer_jwt(&request)?) else {
		return Err(ICError::HEADER_VALIDATION);
	};

	// Check if the user is banned (via Redis)
	check_temp_ban(ajwt.sub, &mut conn_r).await?;


	// Grab needed data from path & query params
	let post_id:i64 = path.0;
	
	// Check the maturity value to make sure it's expected
	// NOTE: i16 because PostgreSQL doesn't use u8/i8
	let maturity:i16 = match params.maturity {
		1 | 2 | 3 | 4 | 5 => params.maturity.into(),
		_ => return Err(ICError::HEADER_VALIDATION),
	};

	// Grab the Bearer header & check it's encoding
	let jwt_string = get_bearer_jwt(&request)?;
	// Decode the JWT & make sure it's ours
	let Ok(ajwt) = AuthJwt::decode_with_defaults(jwt_string) else {
		return Err(ICError::HEADER_VALIDATION);
	};

	// Get the DB connection
	let conn = postgres.get_conn().await?;
	// TODO - This doesn't work yet
	//check_perm_ban(ajwt.sub, &conn).await?;


	match conn.query_typed_one(
		"SELECT SoftMod.SetMaturityVote(post_id:=$1, user_id:=$2, maturity:=$3, is_sexual:=$4, is_gore:=$5, is_trauma:=$6)",
		&[
			(&post_id,  Type::INT8),
			(&ajwt.sub, Type::INT8),
			(&maturity,  Type::INT2),
			(&params.is_sexual, Type::BOOL),
			(&params.is_gore,   Type::BOOL),
			(&params.is_trauma, Type::BOOL),
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