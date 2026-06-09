use std::sync::LazyLock;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
	//get_current_timestamp,
	decode,
	decode_header,
	//crypto::verify,
};
use jsonwebtoken::{
	Algorithm,
	DecodingKey,
	Validation,
	errors::Result,
};

const ALGO:Algorithm = Algorithm::EdDSA;

fn get_iss_aud() -> (String, String) {
	return (
		std::env::var("IC_JWT_ISS")
			.or(std::env::var("IC_ORIGIN"))
			.expect("Could not parse envvar: IC_JWT_ISS/IC_ORIGIN"),
		std::env::var("IC_JWT_AUD")
			.or(std::env::var("IC_ORIGIN"))
			.expect("Could not parse envvar: IC_JWT_AUD/IC_ORIGIN"),
	)
}


static DECODE_KEY:LazyLock<DecodingKey> = LazyLock::new(||{
	let data:Vec<u8> = std::fs::read(
		std::env::var("IC_JWT_PUB").unwrap()).unwrap();

	// TODO: Check the file to specify the algorithm
	return match ALGO {
		Algorithm::HS256 |
		Algorithm::HS384 |
		Algorithm::HS512
			=> DecodingKey::from_secret(&data),
		Algorithm::ES256 |
		Algorithm::ES384
			=> DecodingKey::from_ec_pem(&data).unwrap(),
		Algorithm::RS256 |
		Algorithm::RS384 |
		Algorithm::RS512 |
		Algorithm::PS256 |
		Algorithm::PS384 |
		Algorithm::PS512
			=> DecodingKey::from_rsa_pem(&data).unwrap(),
		Algorithm::EdDSA
			=> DecodingKey::from_ed_pem(&data).unwrap(),
	};
});

// If specified, check for an older public key. 
static DECODE_KEY_ROTATED:LazyLock<Option<DecodingKey>> = LazyLock::new(||{
	let Ok(file) = std::env::var("IC_JWT_PUB_ROTATED") else {
		return None;
	};
	let Ok(data) = std::fs::read(file) else {
		return None;
	};

	// TODO: Check the file to specify the algorithm
	return match ALGO {
		Algorithm::HS256 |
		Algorithm::HS384 |
		Algorithm::HS512
			=> Some(DecodingKey::from_secret(&data)),
		Algorithm::ES256 |
		Algorithm::ES384
			=> DecodingKey::from_ec_pem(&data).ok(),
		Algorithm::RS256 |
		Algorithm::RS384 |
		Algorithm::RS512 |
		Algorithm::PS256 |
		Algorithm::PS384 |
		Algorithm::PS512
			=> DecodingKey::from_rsa_pem(&data).ok(),
		Algorithm::EdDSA
			=> DecodingKey::from_ed_pem(&data).ok(),
	};
});


// The encode/decode functions are identical, so toss them here
pub trait DecodeJwt {
	fn decode(jwt:&str) -> Result<Self>
			where Self:Sized + for<'a> Deserialize<'a> {
		let (iss, aud) = get_iss_aud();
		
		let header = decode_header(&jwt)?;

		let mut validation = Validation::new(header.alg);
		validation.set_issuer(&[iss]);
		validation.set_audience(&[aud]);

		return match decode(&jwt, &DECODE_KEY, &validation) {
			Ok(token) => return {
				Ok(token.claims)
			},
			Err(e) => {
				// If we can't validate, check if we have an older,
				// rotated key we can use instead...
				if let Some(key) = DECODE_KEY_ROTATED.as_ref() {
					let token = decode(&jwt, &key, &validation)?;
					Ok(token.claims)
				} else {
					// Else return the original error - we tried
					Err(e)
				}
			},
		};
	}
}


// A short-lived JWT, containing lots of pieces of information
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthJwt {
	pub iss: String,
	pub aud: String,
	pub exp: u64,
	pub sub: i64,     // account_id
	pub user: String, // username
	pub claims: Vec<String>, // List of claims & roles
}


impl DecodeJwt for AuthJwt {}


// This will validate any of the JWTs without us knowing which one
//pub fn validate(jwt_string:&str) -> bool {
//	// Split the JWT (ie. header.claims.signature) into the message (header.claims) & the signature
//	let Some((msg, sig)) = jwt_string.rsplit_once('.') else {return false};
//	return verify(&sig, msg.as_bytes(), &DECODE_KEY, ALGO).ok()
//		.or_else(|| {
//			if let Some(key) = DECODE_KEY_ROTATED.as_ref() {
//				verify(&sig, msg.as_bytes(), &key, ALGO).ok()
//			} else {
//				None
//			}
//		})
//		.unwrap_or(false);
//}