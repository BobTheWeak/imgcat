use std::sync::LazyLock;

use jsonwebtoken::{Algorithm, DecodingKey};
#[cfg(feature="encode")]
use jsonwebtoken::{EncodingKey};

// TODO: Make this configurable. But... do we really need to?
pub(crate) const ALGO:Algorithm = Algorithm::EdDSA;

#[cfg(feature="std_envvars")]
pub(crate) static ISSUER:LazyLock<String> = LazyLock::new(||{
	std::env::var("IC_JWT_ISS")
		.expect("Could not parse envvar: IC_JWT_ISS")
});

#[cfg(feature="std_envvars")]
pub(crate) static AUDIENCE:LazyLock<String> = LazyLock::new(||{
	std::env::var("IC_JWT_AUD")
		.expect("Could not parse envvar: IC_JWT_AUD")
});

// TODO: LazyLock is NOT the right data structure. It can only be set once,
// and then requires a service restart. Do a proper cache instead.

#[cfg(all(feature="encode", feature="std_envvars"))]
pub(crate) static ENCODE_KEY:LazyLock<EncodingKey> = LazyLock::new(||{

	let key = &std::env::var("IC_JWT_PVT").expect("Could not parse envvar: IC_JWT_PVT");
	let data = if std::path::Path::new(&key).exists() {
		std::fs::read(&key).expect("IC_JWT_PVT looks like a file, but could not be read")
	} else {
		key.clone().into_bytes()
	};

	// TODO: Check the file to specify the algorithm
	return match ALGO {
		Algorithm::HS256 |
		Algorithm::HS384 |
		Algorithm::HS512
			=> EncodingKey::from_secret(&data),
		Algorithm::ES256 |
		Algorithm::ES384
			=> EncodingKey::from_ec_pem(&data).unwrap(),
		Algorithm::RS256 |
		Algorithm::RS384 |
		Algorithm::RS512 |
		Algorithm::PS256 |
		Algorithm::PS384 |
		Algorithm::PS512
			=> EncodingKey::from_rsa_pem(&data).unwrap(),
		Algorithm::EdDSA
			=> EncodingKey::from_ed_pem(&data).unwrap(),
	};
});

#[cfg(feature="std_envvars")]
pub(crate) static DECODE_KEY:LazyLock<DecodingKey> = LazyLock::new(||{
	
	let key = &std::env::var("IC_JWT_PUB").expect("Could not parse envvar: IC_JWT_PUB");
	let data = if std::path::Path::new(&key).exists() {
		std::fs::read(&key).expect("IC_JWT_PUB looks like a file, but could not be read")
	} else {
		key.clone().into_bytes()
	};

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

// Optional: Specify the older, previous, public key, so pre-rotation clients can still refresh tokens
// NOTE: While this is optional, in practice the server config guarantees we have a value
#[cfg(feature="std_envvars")]
pub(crate) static DECODE_KEY_ROTATED:LazyLock<Option<DecodingKey>> = LazyLock::new(||{
	
	let Ok(key) = &std::env::var("IC_JWT_PUB_ROTATED") else { return None };
	let data = if std::path::Path::new(&key).exists() {
		std::fs::read(&key).expect("IC_JWT_PUB_ROTATED looks like a file, but could not be read")
	} else {
		key.clone().into_bytes()
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