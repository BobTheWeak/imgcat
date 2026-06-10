use serde::{Deserialize};

use jsonwebtoken::{
	DecodingKey,
	Validation,
	errors::Result,
	decode_header,
	decode,
};

#[cfg(feature="std_envvars")]
use crate::keys::{DECODE_KEY, DECODE_KEY_ROTATED, ISSUER, AUDIENCE};

pub trait DecodeJwt {
	fn decode(
			jwt:&str,
			key:&DecodingKey,
			key_rot:Option<&DecodingKey>,
			iss:Vec<&'static str>,
			aud:Vec<&'static str>,
		) -> Result<Self>
			where Self:Sized + for<'a> Deserialize<'a> {
		
		let header = decode_header(&jwt)?;

		let mut validation = Validation::new(header.alg);
		validation.set_issuer(&iss);
		validation.set_audience(&aud);

		return match decode(&jwt, key, &validation) {
			Ok(token) => return {
				Ok(token.claims)
			},
			Err(e) => {
				// If we can't validate, check if we have an older,
				// rotated key we can use instead...
				if let Some(key) = key_rot.as_ref() {
					let token = decode(&jwt, &key, &validation)?;
					Ok(token.claims)
				} else {
					// Else return the original error - we tried
					Err(e)
				}
			},
		};
	}

	#[cfg(feature="std_envvars")]
	fn decode_with_defaults(jwt:&str) -> Result<Self>
			where Self:Sized + for<'a> Deserialize<'a> {
		Self::decode(jwt, &DECODE_KEY, DECODE_KEY_ROTATED.as_ref(), vec!(&ISSUER, ), vec!(&AUDIENCE, ))
	}
}