use serde::Deserialize;
use actix_web::{HttpRequest};
use actix_web::web::{Path, Query, Data};

use ic_actix::{ICResult, ICError, AppStatePostgres, AppStateRedis, get_bearer_jwt, get_user_ip};
use postgres_types::Type;


#[derive(Deserialize)]
pub struct VoteReviewParams {
	#[serde(rename="c")]
	comment: String
}


pub async fn vote_review_anon(
		path: Path<(i64,)>,
		params: Query<VoteReviewParams>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		request: HttpRequest,
	) -> ICResult<()> {

	let mut conn_r = redis.get_conn()?;

	// Get user IP for rate-limiting (For an anon request, IP is required)
	let Some(user_ip) = get_user_ip(request.headers())? else {
		return Err(ICError::HEADER_MISSING);
	};
	redis.check_rate_limit_conn(user_ip, &mut conn_r)?;

	// Grab the Bearer header & decode it into an AuthJwt
	if let Ok(_jwt_string) = get_bearer_jwt(&request) {
		// If we have a JWT, then how/why did we get here?
		// Do we forward this requrest to the non-anon version?
		// TODO: Figure this logic out. Maybe we just combine the two calls & do the right thing?
		return Err(ICError::HEADER_VALIDATION);
	}


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
		"SELECT SoftMod.SetReviewVoteAnon(post_id:=$1, user_ip_address:=$2, comment:=$3)",
		&[
			(&post_id, Type::INT8),
			(&user_ip, Type::INET),
			(&params.comment, Type::TEXT),
		]
	).await {
		Ok(data) => {
			// The function returns BOOL
			let result:bool = data.get(0);
			if result {
				return Ok(());
			} else {
				println!("SoftMod.SetReviewVoteAnon returned FALSE. This should never happen.");
				return Err(ICError::error(""));
			}
		},
		Err(e) => {
			println!("{}", e);
			return Err(ICError::POSTGRES_CONN);
		}
	};
}