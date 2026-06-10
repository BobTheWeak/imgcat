use serde::{Serialize, Deserialize};

#[cfg(feature="encode")]
use jsonwebtoken::{get_current_timestamp};

#[cfg(all(feature="encode", feature="std_envvars"))]
use crate::keys::{ISSUER, AUDIENCE};

use crate::decode_jwt::DecodeJwt;
#[cfg(feature="encode")]
use crate::encode_jwt::EncodeJwt;

// A short-lived JWT, containing core data the application needs
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthJwt {
	pub iss: String,
	pub aud: String,
	pub exp: u64,
	pub sub: i64,     // account_id
	pub user: String, // username
	pub claims: Vec<String>, // List of claims & roles
}

impl AuthJwt {
	#[cfg(feature="encode")]
	pub fn new(account_id:i64, username:&str, claims:&Vec<String>, iss:&str, aud:&str) -> Self {
		Self {
			iss: iss.to_string(),
			aud: aud.to_string(),
			exp: get_current_timestamp() + 5*60, // 5min
			sub: account_id,
			user: username.to_string(),
			claims: claims.clone(),
		}
	}

	#[cfg(all(feature="encode", feature="std_envvars"))]
	pub fn new_with_defaults(account_id:i64, username:&str, claims:&Vec<String>) -> Self {
		Self {
			iss: ISSUER.clone(),
			aud: AUDIENCE.clone(),
			exp: get_current_timestamp() + 5*60, // 5min
			sub: account_id,
			user: username.to_string(),
			claims: claims.clone(),
		}
	}
}

impl DecodeJwt for AuthJwt {}

#[cfg(feature="encode")]
impl EncodeJwt for AuthJwt {}