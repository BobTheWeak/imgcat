//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::post;
use actix_web::web::{Form, Data};
use actix_web::{HttpRequest, HttpResponse};

use crate::app_state::AppStateProviders;
use crate::libredis::AppStateRedis;
use crate::libpostgres::AppStatePostgres;
use crate::libjwt::{RefreshJwt, AuthJwt, SignupJwt, DecodeJwt};

use crate::login_helpers::{validate_bearer_auth, get_refresh_jwt, get_auth_jwt, get_refresh_cookie, get_auth_cookie};

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
	let jwt_string = match validate_bearer_auth(&request) {
		Ok(v) => v, Err(e) => return e.into()
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
		return HttpResponse::ServiceUnavailable()
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
	//return send_redirect(Some("/".to_string()), Some(&rjwt), Some(&ajwt), None);

	return HttpResponse::Created() // 201
		.insert_header(("Location", "/home"))
		.cookie(get_refresh_cookie(&rjwt))
		.cookie(get_auth_cookie(&ajwt))
		.finish();
}
