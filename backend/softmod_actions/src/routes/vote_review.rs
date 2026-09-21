use serde::Deserialize;
use actix_web::{HttpRequest};
use actix_web::web::{Path, Query, Data};

use ic_actix::{ICResult, ICError, AppStatePostgres, AppStateRedis, get_bearer_jwt, check_temp_ban, get_user_ip};
use ic_jwt::{AuthJwt, DecodeJwt};
use postgres_types::Type;


#[derive(Deserialize)]
pub struct VoteReviewParams {
	#[serde(rename="c")]
	comment: String
}


pub async fn vote_review(
		path: Path<(i64,)>,
		params: Query<VoteReviewParams>,
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

	// TODO: It's UTF-8, so we should check byte-length, not character-length
	if params.comment.len() > 127 {
		return Err(ICError::error("Comment too long"));
	}


	// Get the DB connection
	let conn = postgres.get_conn().await?;
	// TODO - This doesn't work yet
	//check_perm_ban(ajwt.sub, &conn).await?;

	match conn.query_typed_one(
		"SELECT SoftMod.SetReviewVote(post_id:=$1, user_id:=$2, comment:=$3)",
		&[
			(&post_id,  Type::INT8),
			(&ajwt.sub, Type::INT8),
			(&params.comment, Type::TEXT),
		]
	).await {
		Ok(data) => {
			// The function returns BOOL
			let result:bool = data.get(0);
			if result {
				return Ok(());
			} else {
				println!("SoftMod.SetReviewVote returned FALSE. This should never happen.");
				return Err(ICError::error(""));
			}
		},
		Err(e) => {
			println!("{}", e);
			return Err(ICError::POSTGRES_CONN);
		}
	};
}