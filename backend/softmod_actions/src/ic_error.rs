#![allow(dead_code)]

use std::fmt;
use actix_web::{HttpResponse, HttpResponseBuilder};
use actix_web::{http::StatusCode, error::ResponseError, body::BoxBody};


pub type ICResult<T> = Result<T, ICError>;

#[derive(Debug)]
pub struct ICError {
	status: u16,
	message: &'static str,
}

impl ICError {
	// Used for defining hardcoded, static, service-level error codes
	const fn hardcoded(status: u16, message: &'static str) -> Self {
		Self {status, message: message}
	}

	// Anything not standardized, or call-specific should use this
	pub fn new(message: &'static str) -> Self {
		Self {status:400, message}
	}

	// If we should never hit this condition, send a 500
	pub fn panic(message: &'static str) -> Self {
		Self {status:500, message}
	}

	// Authentication errors (ie. the Bearer token is missing or expired)
	pub const HEADER_MISSING:ICError = ICError::hardcoded(401, "Auth header");
	pub const HEADER_VALIDATION:ICError = ICError::hardcoded(403, "Auth validation");
	pub const BAN_IP:ICError = ICError::hardcoded(418, "Temporary IP ban");
	pub const BAN_TEMP:ICError = ICError::hardcoded(418, "Temporary user ban");
	pub const BAN_PERM:ICError = ICError::hardcoded(418, "Permanent user ban");
	pub const RATE_LIMIT:ICError = ICError::hardcoded(429, "Rate limit");

	// Service errors (ie. Postgres isn't available)
	pub const POSTGRES_ERROR:ICError = ICError::hardcoded(503, "Postgres connection");
	pub const REDIS_ERROR:ICError = ICError::hardcoded(503, "Postgres connection");
}


impl fmt::Display for ICError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "{}", self.message)
	}
}

impl Into<HttpResponse> for ICError {
	fn into(self) -> HttpResponse {
		let code = StatusCode::from_u16(self.status).unwrap();
		HttpResponseBuilder::new(code)
			.insert_header(("IC-Error",self.message))
			.finish()
	}
}

impl ResponseError for ICError {
	fn error_response(&self) -> HttpResponse<BoxBody> {
		// Ignoring whatever defaults BoxBody does. Don't make me write
		// a graphical debugging interface for a purely backend data API.
		HttpResponseBuilder::new(self.status_code())
			// We attach a header with a short message of whats going on.
			// I thought about a GUID or error code, but... the code is
			// open source and it doesn't really matter. Messages are short
			// and cryptic, but we don't have to be pointlessly difficult.
			.insert_header(("IC-Error",self.message))
			.finish()
	}

	fn status_code(&self) -> StatusCode {
		StatusCode::from_u16(self.status).unwrap()
	}
}

