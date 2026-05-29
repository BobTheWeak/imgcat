//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::post;
use actix_web::web::{Form, Data};
use actix_web::{HttpRequest, HttpResponse};

use crate::app_state::AppStateProviders;
use crate::libredis::AppStateRedis;
use crate::libpostgres::AppStatePostgres;
use crate::libjwt::{RefreshJwt, AuthJwt, SignupJwt, DecodeJwt};

use crate::login_helpers::{get_refresh_jwt, get_auth_jwt, send_redirect};

#[derive(Debug, Deserialize)]
struct CreateFormData {
	prv: String, // These must match the jwt
	sub: String, // These must match the jwt
	user: String
}


#[post("/create")]
pub async fn create(
		metadata: Data<AppStateProviders>,
		postgres: Data<AppStatePostgres>,
		redis: Data<AppStateRedis>,
		form: Form<CreateFormData>,
		request: HttpRequest
	) -> HttpResponse {

	// TODO: Rate Limiter

	// Grab the Bearer header & check it's encoding
	let Some(jwt_string) = request.headers().get("Authorization") else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header")).finish();
	};
	let Ok(jwt_string) = jwt_string.to_str() else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header")).finish();
	};
	let Some(jwt_string) = jwt_string.strip_prefix("Bearer ") else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header")).finish();
	};

	// Decode the JWT & make sure it's ours
	let Ok(sjwt) = SignupJwt::decode(jwt_string) else {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Header validation")).finish();
	};

	// Verify the data sent via form & stored in the JWT is identical
	// If anyone's trying sneaky stuff, I suspect it'll be here
	if sjwt.prv != form.prv || sjwt.sub != form.sub {
		return HttpResponse::Forbidden() // 403
			.insert_header(("IC-Error","Form validation")).finish();
	};
	//if let Some((cc, age)) = sjwt.age_ver {
	//	if cc != form.cc || age != form.age {
	//		return HttpResponse::Forbidden() // 403
	//			.insert_header(("IC-Error","Form validation")).finish();
	//	}
	//}

	// Make sure we recognize the provider
	if !metadata.contains_key(&sjwt.prv.clone()) {
		return HttpResponse::BadRequest()
			.insert_header(("IC-Error","Provider")).finish();
	};


	/////////////////////////////
	//   Validation Complete   //
	/////////////////////////////


	let Ok(account_id) = postgres.create_account(&sjwt.prv, &sjwt.sub, &form.user, &sjwt.age_ver).await else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Postgres connection")).finish();
	};
	let Some(account_id) = account_id else {
		return HttpResponse::InternalServerError()
			.insert_header(("IC-Error","Postgres acct creation")).finish();
	};

	let rjwt:RefreshJwt = match get_refresh_jwt(account_id, &redis, &postgres).await {
		Ok(v) => v, Err(e) => return e.into()
	};
	let ajwt:AuthJwt = match get_auth_jwt(&rjwt, &redis, &postgres).await {
		Ok(v) => v, Err(e) => return e.into()
	};
	return send_redirect(Some("/".to_string()), Some(&rjwt), Some(&ajwt), None);
}
