use serde::{Serialize};
use jsonwebtoken::{
	EncodingKey,
	Header,
	errors::Result,
	encode
};

use crate::common::ALGO;

#[cfg(feature="std_envvars")]
use crate::keys::{ENCODE_KEY};

pub trait EncodeJwt {
	fn encode(&self, key:&EncodingKey) -> Result<String>
			where Self:Sized + Serialize {
		return encode(&Header::new(ALGO), self, key);
	}

	#[cfg(feature="std_envvars")]
	fn encode_with_defaults(&self) -> Result<String>
			where Self:Sized + Serialize {
		Self::encode(&self, &ENCODE_KEY)
	}
}