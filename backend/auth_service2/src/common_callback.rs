//use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse};
//use actix_web::web::{Data, Query};
use ic_jwt::{AuthJwt, RefreshJwt, SignupJwt};//, DecodeJwt, EncodeJwt, AgeVerification};
//use ic_actix::{AppStateRedis, AppStatePostgres};
use ic_actix::{ICError, ICResult, ToCookie};
//use ic_actix::{get_bearer_jwt};


use crate::postgres_helpers::{get_account_id, get_account_data};

use deadpool_postgres::tokio_postgres::{Client};
use redis::{Connection};

pub(crate) async fn handle_callback(
		p_conn: &mut Client,
		r_conn: &Connection,
		provider:&str,
		subject:&str,
		access_token:&str,
		redirect_url:Option<String>
	) -> ICResult<HttpResponse> {

	// Grab the account_id
	let account_id:Option<i64> = get_account_id(p_conn, &provider, &subject).await?;

	// Check if we already have an account
	if let Some(account_id) = account_id {
		// They have an account!

		// Grab the data to create JWTs
		let Some(account_data) = get_account_data(p_conn, account_id).await? else {
			// Should never happen, we just returned the id
			return Err(ICError::panic("Postgres data error"));
		};
		
		let rjwt:RefreshJwt = RefreshJwt::new_with_defaults(account_id);
		let ajwt:AuthJwt = AuthJwt::new_with_defaults(
			account_data.account_id,
			account_data.username.as_ref(),
			&account_data.claims,
		);

		return Ok(
			HttpResponse::Ok()
				.insert_header(("Location",
					redirect_url
					.or(Some(std::env::var("IC_ORIGIN").expect("Could not parse envvar: IC_ORIGIN")))
					.unwrap()
					))
				.cookie(rjwt.to_cookie())
				.cookie(ajwt.to_cookie())
				.finish()
		);
		//return Ok(send_redirect(redirect_url, Some(&rjwt), Some(&ajwt), None));

	} else {
		// No account. Redirect them to the new account creation screen.
		
		// If we need to check their age, do this now.
		//let age_ver:Option<AgeVerification> = {
		//	if let Some(country_code) = country_code {
		//		// TODO: I'm not sure how to get the state-code... For now, pass None.
		//		let Ok(needed) = is_age_needed_on_signup(&p_conn, country_code, None).await else {
		//			return Err(ICError::POSTGRES_CONN);
		//			//return HttpResponse::ServiceUnavailable() // 503
		//			//	.insert_header(("IC-Error","Postgres connection")).finish();
		//		};
		//
		//		if !needed {
		//			// Most common path - No age-verification needed
		//			None
		//		} else {
		//			let age:Option<u16> = getage(access_token.secret()).await;
		//
		//			// NOTE: If they deny permission, we return age=0
		//			// TODO: At the moment, we have no way of determining the user's state
		//			Some(AgeVerification::new (country_code, None, age.unwrap_or(0_u16)))
		//		}
		//	} else {
		//		None // Cloudflare can't pinpoint their location (rare)
		//	}
		//};

		// Give them a cookie with provider info, so we can apply that on signup
		let sjwt = SignupJwt::new_with_defaults(provider, subject, None);

		return Ok(
			HttpResponse::Ok()
				.insert_header(("Location", std::env::var("IC_ORIGIN").unwrap() + "/signup"))
				.cookie(sjwt.to_cookie())
				.finish()
		);
		//return Ok(send_redirect(Some("/signup".to_string()), None, None, Some(&sjwt)));
	}
}