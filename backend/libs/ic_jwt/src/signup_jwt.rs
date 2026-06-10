use serde::{Serialize, Deserialize};

#[cfg(feature="encode")]
use jsonwebtoken::{get_current_timestamp};

#[cfg(all(feature="encode", feature="std_envvars"))]
use crate::keys::{ISSUER, AUDIENCE};

use crate::decode_jwt::DecodeJwt;
#[cfg(feature="encode")]
use crate::encode_jwt::EncodeJwt;

use crate::age_verification::AgeVerification;

// A short-term JWT, used for storing the login-provider subject & access-token
// On account-creation, it prevents the user from logging in again.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupJwt {
	pub iss: String,
	pub aud: String,
	pub exp: u64,
	pub sub: String, // The provider subject, not ImgCat's
	pub prv: String, // The provider (ie. "google")
	// NOTE: If this value is None, we don't need to age verify
	pub age_ver: Option<AgeVerification>, // country_code, state (US only?), age
}

impl SignupJwt {
	#[cfg(feature="encode")]
	pub fn new(provider:&str, subject:&str, age_ver: Option<AgeVerification>, iss:&str, aud:&str) -> Self {
		Self {
			iss: iss.to_string(),
			aud: aud.to_string(),
			exp: get_current_timestamp() + 20*60, // 20min
			sub: subject.to_string(),
			prv: provider.to_string(),
			age_ver,
		}
	}

	#[cfg(all(feature="encode", feature="std_envvars"))]
	pub fn new_with_defaults(provider:&str, subject:&str, age_ver: Option<AgeVerification>) -> Self {
		Self {
			iss: ISSUER.clone(),
			aud: AUDIENCE.clone(),
			exp: get_current_timestamp() + 20*60, // 20min
			sub: subject.to_string(),
			prv: provider.to_string(),
			age_ver,
		}
	}
}

impl DecodeJwt for SignupJwt {}

#[cfg(feature="encode")]
impl EncodeJwt for SignupJwt {}