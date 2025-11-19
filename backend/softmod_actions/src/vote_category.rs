use serde::Deserialize;
use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::DB;
use crate::deserialize_helpers::bool_from_int;
use crate::header_helpers::get_user_id;
use crate::conn_helpers::connect;



#[derive(Deserialize)]
struct VoteCategoryParams {
	#[serde(rename="p",deserialize_with="bool_from_int",default)]
	is_politics: bool,
	#[serde(rename="t",deserialize_with="bool_from_int",default)]
	is_thirst_trap: bool,
	#[serde(rename="c",deserialize_with="bool_from_int",default)]
	is_creator: bool,
}

/// A registered user believes this post is one of several divisive categories
#[post("/vote_category/{post_id}")] // Plus optional params: p=1, t=1, c=1
pub async fn vote_category(path: web::Path<(u64,)>, params: web::Query<VoteCategoryParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	let Some(user_id) = get_user_id(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	let query = sqlx::query::<DB>("CALL Content.VoteForCategory(?, ?, ?, ?, ?);")
		.bind(user_id)
		.bind(post_id)
		.bind(params.is_politics)
		.bind(params.is_thirst_trap)
		.bind(params.is_creator);

	if let Ok(conn) = connect().await {
		if let Err(err) = query.execute(conn).await {
			println!("SQL Error: {}", err);
			return HttpResponse::InternalServerError().finish(); // 500
		};
		return HttpResponse::Ok().finish();
	} else {
		println!("SQL Connection failed");
		return HttpResponse::InternalServerError().finish(); // 500
	};
}



