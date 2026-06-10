#![allow(dead_code)]

// Private
#[cfg(feature="std_envvars")] mod keys;

// Traits
mod decode_jwt;
pub use decode_jwt::{DecodeJwt};
#[cfg(feature="encode")] mod encode_jwt;
#[cfg(feature="encode")] pub use encode_jwt::{EncodeJwt};

// Helpers
mod common;
pub use common::{validate};

// Classes
mod auth_jwt;
pub use auth_jwt::{AuthJwt};
#[cfg(feature="refresh")] mod refresh_jwt;
#[cfg(feature="refresh")] pub use refresh_jwt::{RefreshJwt};
#[cfg(feature="signup")] mod age_verification;
#[cfg(feature="signup")] pub use age_verification::{AgeVerification};
#[cfg(feature="signup")] mod signup_jwt;
#[cfg(feature="signup")] pub use signup_jwt::{SignupJwt};