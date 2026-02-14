use serde::Deserialize;
use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::DB;
use crate::deserialize_helpers::bool_from_int;
use crate::header_helpers::get_user_id;
use crate::conn_helpers::connect;



#[derive(Deserialize)]
struct VoteMatureParams {
	#[serde(rename="m")]
	maturity: u8,
	#[serde(rename="s",deserialize_with="bool_from_int",default)]
	is_sexual: bool,
	#[serde(rename="g",deserialize_with="bool_from_int",default)]
	is_gore: bool,
	#[serde(rename="t",deserialize_with="bool_from_int",default)]
	is_trauma: bool,
}


/// A registered user believes this post is mature, spicy, etc.
#[post("/vote_mature/{post_id}")] // Plus required params: m=[0-4], and optional: s=1, g=1, t=1
pub async fn vote_mature(path: web::Path<(u64,)>, params: web::Query<VoteMatureParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	let Some(user_id) = get_user_id(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};
	if params.maturity > 4 {
		//return HttpResponse::BadRequest().into(); // 400
		return HttpResponse::ImATeapot().into(); // 418 -- DEBUG
		
	}

	// NOTE: Will this work for both MariaDB and PostgreSQL?
	let query = sqlx::query::<DB>("CALL Content.VoteForMaturity(?, ?, ?, ?, ?, ?);")
		.bind(user_id)
		.bind(post_id)
		// TODO: Accepted values are [0-4] via table contstraint. Should be checked here too.
		.bind(params.maturity)
		.bind(params.is_sexual)
		.bind(params.is_gore)
		.bind(params.is_trauma);
	
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
//
	
	
	//} else {
	//	println!("SQL Connection failed");
	//	return HttpResponse::InternalServerError().finish(); // 500
	//}
}