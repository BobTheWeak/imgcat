use serde::Deserialize;
use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::DB;
use crate::deserialize_helpers::bool_from_int;
use crate::header_helpers::get_bearer_auth;
use crate::conn_helpers::connect;
use crate::libjwt::{AuthJwt, DecodeJwt};


#[derive(Deserialize)]
struct VoteCategoryParams {
	#[serde(rename="a",deserialize_with="bool_from_int",default)]
	is_animals: bool,
	#[serde(rename="c",deserialize_with="bool_from_int",default)]
	is_creator: bool,
	#[serde(rename="s",deserialize_with="bool_from_int",default)]
	is_selfie: bool,
	#[serde(rename="n",deserialize_with="bool_from_int",default)]
	is_news: bool,
	#[serde(rename="p",deserialize_with="bool_from_int",default)]
	is_politics: bool,
	#[serde(rename="r",deserialize_with="bool_from_int",default)]
	is_ai: bool,
}

/// A registered user believes this post is one of several divisive categories
#[post("/vote_category/{post_id}")] // Plus optional params: p=1, n=1, etc.
pub async fn vote_category(path: web::Path<(u64,)>, params: web::Query<VoteCategoryParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	
	// Grab the Bearer header & check it's encoding
	let jwt_string = match get_bearer_auth(&request) {
		Ok(v) => v, Err(e) => return e.into()
	};

	// Decode the JWT & make sure it's ours
	let Ok(ajwt) = AuthJwt::decode(jwt_string) else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header validation")).finish();
	};

	let query = sqlx::query::<DB>("CALL Content.VoteForCategory(?, ?, ?, ?, ?);")
		.bind(ajwt.sub)
		.bind(post_id)
		.bind(params.is_politics)
		.bind(params.is_selfie)
		.bind(params.is_creator);

	if let Ok(conn) = connect().await {
		if let Err(err) = query.execute(&conn).await {
			println!("SQL Error: {}", err);
			return HttpResponse::InternalServerError().finish(); // 500
		};
		return HttpResponse::Ok().finish();
	} else {
		println!("SQL Connection failed");
		return HttpResponse::InternalServerError().finish(); // 500
	};
}



