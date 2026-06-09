use serde::Deserialize;
use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::DB;
use crate::deserialize_helpers::bool_from_int;
use crate::header_helpers::get_bearer_auth;
use crate::conn_helpers::connect;
use crate::libjwt::{AuthJwt, DecodeJwt};


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
	
	// Grab the Bearer header & check it's encoding
	let jwt_string = match get_bearer_auth(&request) {
		Ok(v) => v, Err(e) => return e.into()
	};

	// Decode the JWT & make sure it's ours
	let Ok(ajwt) = AuthJwt::decode(jwt_string) else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header validation")).finish();
	};

	if params.maturity > 4 {
		//return HttpResponse::BadRequest().into(); // 400
		return HttpResponse::ImATeapot().into(); // 418 -- DEBUG
		
	}

	// NOTE: Will this work for both MariaDB and PostgreSQL?
	let query = sqlx::query::<DB>("CALL Content.VoteForMaturity(?, ?, ?, ?, ?, ?);")
		.bind(ajwt.sub)
		.bind(post_id)
		// TODO: Accepted values are [0-4] via table contstraint. Should be checked here too.
		.bind(params.maturity)
		.bind(params.is_sexual)
		.bind(params.is_gore)
		.bind(params.is_trauma);
	
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
//
	
	
	//} else {
	//	println!("SQL Connection failed");
	//	return HttpResponse::InternalServerError().finish(); // 500
	//}
}