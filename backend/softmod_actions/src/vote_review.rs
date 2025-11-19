use serde::Deserialize;
use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::DB;
use crate::header_helpers::get_user_id;
use crate::conn_helpers::connect;



#[derive(Deserialize)]
struct VoteReviewParams {
	#[serde(rename="c")]
	comment: String
}


/// A registered user has flagged this post for moderators to review
#[post("/vote_review/{post_id}")] // Plus required params: c=<Reason I'm reporting this post>
pub async fn vote_review(path: web::Path<(u64,)>, params: web::Query<VoteReviewParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	let Some(user_id) = get_user_id(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	// TODO: It's UTF-8, so we should check byte-length, not character-length
	if params.comment.len() > 256 {
		return HttpResponse::BadRequest().into(); // 400
	}

	let query = sqlx::query::<DB>("CALL Content.FlagForReview(?, ?, ?);")
		.bind(user_id)
		.bind(post_id)
		.bind(params.comment.clone());

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








//
//
//
//
///// A registered user has flagged this post for moderators to review
//#[post("/vote_review/{post_id}")] // Plus required params: c=<Reason I'm reporting this post>
//async fn vote_review(path: web::Path<(u64,)>, params: web::Query<VoteReviewParams>, request: HttpRequest) -> HttpResponse {
//	// Grab needed data from path & headers
//	let post_id:u64 = path.0;
//	let Some(user_id) = get_user_id(&request) else {
//		return HttpResponse::Forbidden().into(); // 403
//	};
//
//	// TODO: It's UTF-8, so we should check byte-length, not character-length
//	if params.comment.len() > 256 {
//		return HttpResponse::BadRequest().into(); // 400
//	}
//
//	if IC_DB_IS_MARIA {
//		if let Ok(mut conn) = MySqlConnectOptions::new()
//				.host(IC_DB_HOST)
//				.username(IC_DB_USER)
//				.password(IC_DB_PASS)
//				.connect().await {
//			let query = sqlx::query("CALL Content.FlagForReview(?, ?, ?);")
//					.bind(user_id)
//					.bind(post_id)
//					.bind(params.comment.clone());
//			if let Err(err) = query.execute(&mut conn).await {
//				println!("SQL Error: {}", err);
//				return HttpResponse::InternalServerError().finish(); // 500
//			};
//			return HttpResponse::Ok().finish();	
//		} else {
//			println!("SQL Connection failed");
//			return HttpResponse::InternalServerError().finish(); // 500
//		}
//	} else {
//		// Connect to PostgreSQL database
//		todo!();
//	}
//
//	//return HttpResponse::Ok().finish(); // 200
//}