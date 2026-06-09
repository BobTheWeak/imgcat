use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::DB;
use crate::header_helpers::get_bearer_auth;
use crate::conn_helpers::connect;
use crate::libjwt::{AuthJwt, DecodeJwt};


/// A registered user believes we should add one or more tags to this post
#[post("/vote_tag/{post_id}")] // Plus params: &t=star_trek&t=trek&t=star%20trek
pub async fn vote_tag(
		path: web::Path<(u64,)>,
		request: HttpRequest
	) -> HttpResponse {
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

	// NOTE: This is some opinionated bullshit... They REMOVED the function that returns a list of pairs
	let query_params:Vec<(String, String)> = web::Query::<Vec<(String, String)>>::from_query(request.query_string()).unwrap().to_vec();
	let tags:Vec<String> = query_params.into_iter().filter(|x| x.0 == "t").map(|x| x.1).collect();

	if tags.len() > 0 {
		if let Ok(pool) = connect().await {
			// Since we're (possibly) running multiple statements, wrap everything
			// inside a transaction. We'll only .await on commit.
			if let Ok(mut tx) = pool.begin().await {
				for t in tags {
					let query = sqlx::query::<DB>("CALL Content.VoteForTag(?, ?, ?);")
						.bind(ajwt.sub)
						.bind(post_id)
						.bind(t);
					let _ = query.execute(&mut *tx).await;
				}
				
				if let Err(err) = tx.commit().await {
					println!("SQL Error: {}", err);
					return HttpResponse::InternalServerError().finish(); // 500
				};
			}
		} else {
			println!("SQL Connection failed");
			return HttpResponse::InternalServerError().finish(); // 500
		};
	}

	return HttpResponse::Ok().finish();
}
