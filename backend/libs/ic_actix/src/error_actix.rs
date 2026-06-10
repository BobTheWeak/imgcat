use actix_web::{HttpResponse, HttpResponseBuilder};
use actix_web::{http::StatusCode, error::ResponseError, body::BoxBody};

use crate::ic_error::ICError;

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