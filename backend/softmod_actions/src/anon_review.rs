use serde::Deserialize;
use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::DB;
use crate::header_helpers::get_user_ip;
use crate::conn_helpers::connect;



#[derive(Deserialize)]
struct VoteReviewParams {
	#[serde(rename="c")]
	comment: String
}


/// A registered user has flagged this post for moderators to review
#[post("/anon_review/{post_id}")] // Plus required params: c=<Reason I'm reporting this post>
pub async fn anon_review(path: web::Path<(u64,)>, params: web::Query<VoteReviewParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	let Some(user_ip_address) = get_user_ip(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	// TODO: It's UTF-8, so we should check byte-length, not character-length
	if params.comment.len() > 256 {
		return HttpResponse::BadRequest().into(); // 400
	}

	let query = sqlx::query::<DB>("CALL Content.FlagForReviewAnon(?, ?, ?);")
		.bind(user_ip_address)
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


// /// An anonymous user has marked this post for review.
// /// It might be spam or AI, or a govt official without an account.
// #[post("/anon_review/{post_id}")]
// async fn anon_review(path: web::Path<(u64,)>, params: web::Query<VoteReviewParams>, request: HttpRequest) -> HttpResponse {
// 	// Grab needed data from path & headers
// 	let post_id:u64 = path.0;
// 	let Some(user_ip_address) = get_user_ip(&request) else {
// 		return HttpResponse::Forbidden().into(); // 403
// 	};
// 
// 	// TODO: It's UTF-8, so we should check byte-length, not character-length
// 	if params.comment.len() > 256 {
// 		return HttpResponse::BadRequest().into(); // 400
// 	}
// 
// 	// NOTE: MariaDB versions: INET6 (10.5), INET4 (10.10), Store INET4 inside INET6 (11.3)
// 	// Min is 10.5 b/c the webservice translates any v4 into a v6 before calling the SP.
// 
// 	if IC_DB_IS_MARIA {
// 
// 		// NOTE: IPv6 is a new(-ish) datatype for MariaDB. It didnt 
// 
// 		if let Ok(mut conn) = MySqlConnectOptions::new()
// 				.host(IC_DB_HOST)
// 				.username(IC_DB_USER)
// 				.password(IC_DB_PASS)
// 				.connect().await {
// 			let query = sqlx::query("CALL Content.FlagForReviewAnon(?, ?, ?);")
// 					.bind(user_ip_address)
// 					.bind(post_id)
// 					.bind(params.comment.clone());
// 			if let Err(err) = query.execute(&mut conn).await {
// 				println!("SQL Error: {}", err);
// 				return HttpResponse::InternalServerError().finish(); // 500
// 			};
// 			return HttpResponse::Ok().finish();	
// 		} else {
// 			println!("SQL Connection failed");
// 			return HttpResponse::InternalServerError().finish(); // 500
// 		}
// 	} else {
// 		// Connect to PostgreSQL database
// 		todo!();
// 	}
// 
// 	//return HttpResponse::Ok().finish(); // 200
// }