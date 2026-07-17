//use std::collections::HashMap;
use serde::Deserialize;
use actix_web::web::{Form, Data};
use actix_web::{HttpRequest, HttpResponse};

use crate::SUPPORTED_PROVIDERS;
//use crate::app_state::AppStateProviders;
//use crate::libredis::AppStateRedis;
use crate::postgres_helpers::{get_account_data, create_account};
use ic_jwt::{RefreshJwt, AuthJwt, SignupJwt, DecodeJwt};
use ic_actix::{ICResult, ICError, AppStatePostgres, ToCookie};
use ic_actix::{get_bearer_jwt};

#[derive(Debug, Deserialize)]
pub(crate) struct CreateFormData {
	prv: String, // These must match the jwt
	sub: String, // These must match the jwt
	user: String
}


pub async fn new_account(
		postgres: Data<AppStatePostgres>,
		//redis: Data<AppStateRedis>,
		form: Form<CreateFormData>,
		request: HttpRequest
	) -> ICResult<HttpResponse> {

	// TODO: Rate Limiter

	// Grab the Signup JWT & make sure check its ours
	let jwt_string = get_bearer_jwt(&request)?;
	let Ok(sjwt) = SignupJwt::decode_with_defaults(jwt_string) else {
		return Err(ICError::HEADER_VALIDATION);
	};

	// Verify the data sent via form & stored in the JWT is identical
	// If anyone's trying sneaky stuff, I suspect it'll be here
	if sjwt.prv != form.prv || sjwt.sub != form.sub {
		return Err(ICError::error("Form validation"));
	};
	//if let Some((cc, age)) = sjwt.age_ver {
	//	if cc != form.cc || age != form.age {
	//		return HttpResponse::Forbidden() // 403
	//			.insert_header(("IC-Error","Form validation")).finish();
	//	}
	//}

	// Make sure we recognize the provider
	if SUPPORTED_PROVIDERS.iter().find(|x| *x == &sjwt.prv).is_none() {
		return Err(ICError::panic("Provider"));
	};


	/////////////////////////////
	//   Validation Complete   //
	/////////////////////////////


	let mut p_conn = postgres.get_conn().await?;
	let account_id = create_account(&p_conn, &sjwt.prv, &sjwt.sub, &form.user, &sjwt.age_ver).await?;

	let Some(account_id) = account_id else {
		// Not sure why we couldn't create an account. Maybe Username validation failed? Or it already exists?
		return Err(ICError::error("Postgres acct creation"));
	};

	// Now, automatically log the new user in
	let account_data = get_account_data(&mut p_conn, account_id).await?;

	let Some(account_data) = account_data else {
		return Err(ICError::panic("Missing data"));
	};

	let rjwt:RefreshJwt = RefreshJwt::new_with_defaults(account_id);
	let ajwt:AuthJwt = AuthJwt::new_with_defaults(
		account_data.account_id,
		account_data.username.as_ref(),
		&account_data.claims,
	);

	return Ok(HttpResponse::Created() // 201
		.insert_header(("Location", "/home"))
		.cookie(rjwt.to_cookie())
		.cookie(ajwt.to_cookie())
		.finish());
}
