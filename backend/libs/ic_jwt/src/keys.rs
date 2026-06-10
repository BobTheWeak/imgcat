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

#[cfg(all(feature="encode", feature="std_envvars"))]
pub(crate) static ENCODE_KEY:LazyLock<EncodingKey> = LazyLock::new(||{
	let filename:&str = &std::env::var("IC_JWT_PVT").expect("Could not parse envvar: IC_JWT_PVT");
	let data:Vec<u8> = std::fs::read(filename).expect("Could not read file specified in: IC_JWT_PVT");

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
	let filename:&str = &std::env::var("IC_JWT_PUB").expect("Could not parse envvar: IC_JWT_PUB");
	let data:Vec<u8> = std::fs::read(filename).expect("Could not read file specified in: IC_JWT_PUB");

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
#[cfg(feature="std_envvars")]
pub(crate) static DECODE_KEY_ROTATED:LazyLock<Option<DecodingKey>> = LazyLock::new(||{
	let Ok(filename) = &std::env::var("IC_JWT_PUB_ROTATED") else {
		return None;
	};
	let Ok(data) = std::fs::read(filename) else {
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