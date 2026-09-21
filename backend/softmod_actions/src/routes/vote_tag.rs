use actix_web::{HttpRequest};
use actix_web::web::{Path, Query, Data};

use ic_actix::{ICResult, ICError, AppStatePostgres, AppStateRedis, get_bearer_jwt, check_temp_ban, get_user_ip};
use ic_jwt::{AuthJwt, DecodeJwt};
use postgres_types::Type;


/// A registered user believes this post is one of several divisive categories
//#[post("/vote_tag/{post_id}")] // Plus a list of tags: t=tag_name&t=other_tag
pub async fn vote_tag(
		path: Path<(i64,)>,
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

	// NOTE: This is some opinionated bullshit... They REMOVED the function that returns a list of pairs
	let query_params:Vec<(String, String)> = Query::<Vec<(String, String)>>::from_query(request.query_string()).unwrap().to_vec();
	let tags:Vec<String> = query_params.into_iter().filter(|x| x.0 == "t").map(|x| x.1).collect();

	if tags.len() == 0 {
		return Err(ICError::error("No tags"));
	}
	if tags.len() > 50 { // Hard-cap is 253 => u8.MAX - 2 reserved
		return Err(ICError::error("Too many tags"));
	}

	// Get the DB connection
	let conn_p = postgres.get_conn().await?;
	// TODO - This doesn't work yet
	//check_perm_ban(ajwt.sub, &conn_p).await?;

	match conn_p.query_typed_one(
		"SELECT SoftMod.SetTagVote(post_id:=$1, user_id:=$2, tag:=$3)",
		&[
			(&post_id,  Type::INT8),
			(&ajwt.sub, Type::INT8),
			(&tags, Type::TEXT_ARRAY),
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