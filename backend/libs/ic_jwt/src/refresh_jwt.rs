use serde::{Serialize, Deserialize};

#[cfg(feature="encode")]
use jsonwebtoken::{get_current_timestamp};

#[cfg(all(feature="encode", feature="std_envvars"))]
use crate::keys::{ISSUER, AUDIENCE};

use crate::decode_jwt::DecodeJwt;
#[cfg(feature="encode")]
use crate::encode_jwt::EncodeJwt;

/// A long-lived JWT, used for periodically refreshing the login
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshJwt {
	pub iss: String,
	pub aud: String,
	pub exp: u64,
	pub sub: i64, // ImgCat account_id
}

impl RefreshJwt {
	#[cfg(feature="encode")]
	pub fn new(account_id:i64, iss:&str, aud:&str) -> Self {
		Self {
			iss: iss.to_string(),
			aud: aud.to_string(),
			exp: get_current_timestamp() + 14*24*60*60, // 2wks
			sub: account_id,
		}
	}

	#[cfg(all(feature="encode", feature="std_envvars"))]
	pub fn new_with_defaults(account_id:i64) -> Self {
		Self {
			iss: ISSUER.clone(),
			aud: AUDIENCE.clone(),
			exp: get_current_timestamp() + 14*24*60*60, // 2wks
			sub: account_id,
		}
	}
}

impl DecodeJwt for RefreshJwt {}

#[cfg(feature="encode")]
impl EncodeJwt for RefreshJwt {}