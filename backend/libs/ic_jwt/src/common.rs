use jsonwebtoken::{DecodingKey, decode_header, crypto::verify};
#[cfg(feature="encode")]
use jsonwebtoken::{Algorithm};

//pub(crate) fn get_iss_aud() -> (String, String) {
//	return (
//		std::env::var("IC_JWT_ISS").expect("Could not parse envvar: IC_JWT_ISS"),
//		std::env::var("IC_JWT_AUD").expect("Could not parse envvar: IC_JWT_AUD"),
//	)
//}

// TODO: Make this configurable. But... do we really need to?
#[cfg(feature="encode")]
pub(crate) const ALGO:Algorithm = Algorithm::EdDSA;

pub fn validate(jwt_string:&str, key:&DecodingKey, key_rot:Option<&DecodingKey>) -> bool {
	// Split the JWT (ie. header.claims.signature) into the message (header.claims) & the signature
	let Ok(header) = decode_header(&jwt_string) else {return false};
	let Some((msg, sig)) = jwt_string.rsplit_once('.') else {return false};
	return verify(&sig, msg.as_bytes(), &key, header.alg).ok()
		.or_else(|| {
			if let Some(key) = key_rot.as_ref() {
				verify(&sig, msg.as_bytes(), &key, header.alg).ok()
			} else {
				None
			}
		})
		.unwrap_or(false);
}