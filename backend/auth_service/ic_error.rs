use actix_web::{HttpResponse, HttpResponseBuilder, http::StatusCode};


pub type HelperResult<T> = Result<T, HelperError>;


pub struct HelperError {
	status: u16,
	message: String,
}

impl HelperError {
	pub fn new(status: u16, message: &str) -> Self {
		Self {status, message: message.to_string()}
	}
}

impl Into<HttpResponse> for HelperError {
	fn into(self) -> HttpResponse {
		let code = StatusCode::from_u16(self.status).unwrap();
		HttpResponseBuilder::new(code)
			.insert_header(("IC-Error",self.message))
			.finish()
	}
}
