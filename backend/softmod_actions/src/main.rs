// NOTE: This pulls ENV variables at compile-time, not run-time
const IC_HEADER_USER_ID:&str = if let Some(result) = option_env!("IC_HEADER_USER_ID") {result} else {"x-ic-user-id"};
const IC_HEADER_USER_IP:&str = if let Some(result) = option_env!("IC_HEADER_USER_IP") {result} else {"x-ic-user-ip"};

// TODO: We're doing this as a bool, but this more correctly done as a feature flag
const IC_DB_IS_MARIA:bool = option_env!("IC_DB_IS_MARIA").is_some(); // If it's set. But TODO, we should check for falsey values
const IC_DB_HOST:&str = env!("IC_DB_HOST");
const IC_DB_USER:&str = env!("IC_DB_USER");
const IC_DB_PASS:&str = env!("IC_DB_PASS");

use std::net::{Ipv4Addr, Ipv6Addr};

use actix_web::{get, /*post,*/ web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{de, Deserialize};


use sqlx::{ConnectOptions};
use sqlx::mysql::{MySqlConnectOptions};
//use sqlx::postgres::{PgConnection, PgConnectOptions};


/// Parses the "x-ic-user-id" header value from requests, returning None if it doesn't exist
fn get_user_id(request: &HttpRequest) -> Option<u64> {
	if let Some(header) = request.headers().get(IC_HEADER_USER_ID) {
		if let Ok(h) = header.to_str() {
			if let Ok(i) = h.parse::<u64>() {
				return Some(i);
			}
		}
	}
	return None;
}

/// Parses the "x-ic-user-ip" header value from requests, returning None if it doesn't exist.
/// If the user is connected via IPv4, this returns the IPv6 version of it.
fn get_user_ip(request: &HttpRequest) -> Option<Ipv6Addr> {
	// NOTE: MariaDB support for IPv4/6 sucks. It can't store/cast a IPv4
	// into an IPv6, until 11.3, or LTS 11.4 (May 2024). I know... SMH
	// Converting all IPv4's into IPv6's is technically a workaround to support
	// all MariaDB LTS versions. But I wouldn't fix this later. It's fine as is.
	if let Some(header) = request.headers().get(IC_HEADER_USER_IP) {
		if let Ok(h) = header.to_str() {
			// Try parsing as an IPV6 first
			if let Ok(i) = h.parse::<Ipv6Addr>() {
				return Some(i);
			} else {
				// Is it an IPv4?
				if let Ok(i) = h.parse::<Ipv4Addr>() {
					return Some(i.to_ipv6_compatible());
				}
			}
		}
	}
	return None;
}

/// Serde deserialization method, for "/url?tag=1"
fn bool_from_int<'a, D>(d:D) -> Result<bool, D::Error> 
where D:de::Deserializer<'a> {
	// TODO: Should recognize "/url?value1&value2" as both set/true
	// It can save a few bytes, but it may complicate logic/testing
	match u8::deserialize(d)? {
		0 => Ok(false),
		1 => Ok(true),
		bad => Err(de::Error::invalid_value(de::Unexpected::Unsigned(bad as u64), &"zero or one")),
	}
}


/// A registered user believes we should add one or more tags to this post
#[get("/vote_tag/{post_id}")] // Plus params: &t=star_trek&t=trek&t=star%20trek
async fn vote_tag(path: web::Path<(u64,)>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	let Some(user_id) = get_user_id(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	// NOTE: This is some opinionated bullshit... They REMOVED the function that returns a list of pairs
	let query_params:Vec<(String, String)> = web::Query::<Vec<(String, String)>>::from_query(request.query_string()).unwrap().to_vec();
	let tags:Vec<String> = query_params.into_iter().filter(|x| x.0 == "t").map(|x| x.1).collect();

	if tags.len() > 0 {
		if IC_DB_IS_MARIA {
			if let Ok(mut conn) = MySqlConnectOptions::new()
					.host(IC_DB_HOST)
					.username(IC_DB_USER)
					.password(IC_DB_PASS)
					.connect().await {
				for t in tags {
					let query = sqlx::query("CALL Content.VoteForTag(?, ?, ?);")
							.bind(user_id)
							.bind(post_id)
							.bind(t);
					if let Err(err) = query.execute(&mut conn).await {
						println!("SQL Error: {}", err);
						return HttpResponse::InternalServerError().finish(); // 500
					};
				}
				return HttpResponse::Ok().finish();	
			} else {
				println!("SQL Connection failed");
				return HttpResponse::InternalServerError().finish(); // 500
			}
		} else {
			// Connect to PostgreSQL database
			todo!();
		}
	}

	return HttpResponse::Ok().finish(); // 200
}



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
#[get("/vote_mature/{post_id}")] // Plus required params: m=[1-5], and optional: s=1, g=1, t=1
async fn vote_mature(path: web::Path<(u64,)>, params: web::Query<VoteMatureParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	let Some(user_id) = get_user_id(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	if IC_DB_IS_MARIA {
		if let Ok(mut conn) = MySqlConnectOptions::new()
				.host(IC_DB_HOST)
				.username(IC_DB_USER)
				.password(IC_DB_PASS)
				.connect().await {
			let query = sqlx::query("CALL Content.VoteForMaturity(?, ?, ?, ?, ?, ?);")
					.bind(user_id)
					.bind(post_id)
					// TODO: Accepted values are [0-4] via table contstraint. Should be checked here too.
					.bind(params.maturity)
					.bind(params.is_sexual)
					.bind(params.is_gore)
					.bind(params.is_trauma);
			if let Err(err) = query.execute(&mut conn).await {
				println!("SQL Error: {}", err);
				return HttpResponse::InternalServerError().finish(); // 500
			};
			return HttpResponse::Ok().finish();	
		} else {
			println!("SQL Connection failed");
			return HttpResponse::InternalServerError().finish(); // 500
		}
	} else {
		// Connect to PostgreSQL database
		todo!();
	}

	//return HttpResponse::Ok().finish(); // 200
}


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
#[get("/vote_category/{post_id}")] // Plus optional params: p=1, t=1, c=1
async fn vote_category(path: web::Path<(u64,)>, params: web::Query<VoteCategoryParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & query params
	let post_id:u64 = path.0;
	let Some(user_id) = get_user_id(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	if IC_DB_IS_MARIA {
		if let Ok(mut conn) = MySqlConnectOptions::new()
				.host(IC_DB_HOST)
				.username(IC_DB_USER)
				.password(IC_DB_PASS)
				.connect().await {
			let query = sqlx::query("CALL Content.VoteForCategory(?, ?, ?, ?, ?);")
					.bind(user_id)
					.bind(post_id)
					.bind(params.is_politics)
					.bind(params.is_thirst_trap)
					.bind(params.is_creator);
			if let Err(err) = query.execute(&mut conn).await {
				println!("SQL Error: {}", err);
				return HttpResponse::InternalServerError().finish(); // 500
			};
			return HttpResponse::Ok().finish();	
		} else {
			println!("SQL Connection failed");
			return HttpResponse::InternalServerError().finish(); // 500
		}
	} else {
		// Connect to PostgreSQL database
		todo!();
	}

	//return HttpResponse::Ok().finish(); // 200
}


#[derive(Deserialize)]
struct VoteReviewParams {
	#[serde(rename="c")]
	comment: String
}

/// A registered user has flagged this post for moderators to review
#[get("/vote_review/{post_id}")] // Plus required params: c=<Reason I'm reporting this post>
async fn vote_review(path: web::Path<(u64,)>, params: web::Query<VoteReviewParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & headers
	let post_id:u64 = path.0;
	let Some(user_id) = get_user_id(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	// TODO: It's UTF-8, so we should check byte-length, not character-length
	if params.comment.len() > 256 {
		return HttpResponse::BadRequest().into(); // 400
	}

	if IC_DB_IS_MARIA {
		if let Ok(mut conn) = MySqlConnectOptions::new()
				.host(IC_DB_HOST)
				.username(IC_DB_USER)
				.password(IC_DB_PASS)
				.connect().await {
			let query = sqlx::query("CALL Content.FlagForReview(?, ?, ?);")
					.bind(user_id)
					.bind(post_id)
					.bind(params.comment.clone());
			if let Err(err) = query.execute(&mut conn).await {
				println!("SQL Error: {}", err);
				return HttpResponse::InternalServerError().finish(); // 500
			};
			return HttpResponse::Ok().finish();	
		} else {
			println!("SQL Connection failed");
			return HttpResponse::InternalServerError().finish(); // 500
		}
	} else {
		// Connect to PostgreSQL database
		todo!();
	}

	//return HttpResponse::Ok().finish(); // 200
}







/// An anonymous user has marked this post for review.
/// It might be spam or AI, or a govt official without an account.
#[get("/anon_review/{post_id}")]
async fn anon_review(path: web::Path<(u64,)>, params: web::Query<VoteReviewParams>, request: HttpRequest) -> HttpResponse {
	// Grab needed data from path & headers
	let post_id:u64 = path.0;
	let Some(user_ip_address) = get_user_ip(&request) else {
		return HttpResponse::Forbidden().into(); // 403
	};

	// NOTE: MariaDB versions: INET6 (10.5), INET4 (10.10), Store INET4 inside INET6 (11.3)
	// Min is 10.5 b/c the webservice translates any v4 into a v6 before calling the SP.

	if IC_DB_IS_MARIA {

		// NOTE: IPv6 is a new(-ish) datatype for MariaDB. It didnt 

		if let Ok(mut conn) = MySqlConnectOptions::new()
				.host(IC_DB_HOST)
				.username(IC_DB_USER)
				.password(IC_DB_PASS)
				.connect().await {
			let query = sqlx::query("CALL Content.FlagForReviewAnon(?, ?, ?);")
					.bind(user_ip_address)
					.bind(post_id)
					.bind(params.comment.clone());
			if let Err(err) = query.execute(&mut conn).await {
				println!("SQL Error: {}", err);
				return HttpResponse::InternalServerError().finish(); // 500
			};
			return HttpResponse::Ok().finish();	
		} else {
			println!("SQL Connection failed");
			return HttpResponse::InternalServerError().finish(); // 500
		}
	} else {
		// Connect to PostgreSQL database
		todo!();
	}

	//return HttpResponse::Ok().finish(); // 200
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(vote_tag)
        .service(vote_mature)
        .service(vote_category)
        .service(vote_review)
        .service(anon_review)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
