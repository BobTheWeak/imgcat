use actix_web::HttpRequest;

use crate::ic_error::{ICError, ICResult};

pub fn get_bearer_jwt(request:&HttpRequest) -> ICResult<&str> {
	let Some(jwt_string) = request.headers().get("Authorization") else {
		return Err(ICError::HEADER_MISSING);
	};
	let Ok(jwt_string) = jwt_string.to_str() else {
		return Err(ICError::HEADER_MISSING);
	};
	let Some(jwt_string) = jwt_string.strip_prefix("Bearer ") else {
		return Err(ICError::HEADER_MISSING);
	};

	Ok(jwt_string)
}