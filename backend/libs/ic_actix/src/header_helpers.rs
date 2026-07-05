use actix_web::HttpRequest;
use actix_web::http::header::HeaderMap;

use crate::ic_error::{ICError, ICResult};


//TODO: This should take the Header obj, not the HttpRequest obj
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


pub fn get_country_code(headers:&HeaderMap) -> ICResult<Option<&str>> {
	if let Some(cc) = headers.get("CF-IPCountry") {
		let Ok(cc) = cc.to_str() else {
			// This parsing could fail if the header has invalid characters
			return Err(ICError::HEADER_MISSING);
		};

		// Block Tor traffic. There are good reasons to allow it, but we need
		// to ensure our security against spam & hackers is rock-solid first
		if cc == "T1" {
			return Err(ICError::BAN_TOR);
		}
		
		if cc != "XX" {
			Ok(Some(cc))
		} else {
			Ok(None)
		}
	} else {
		Ok(None)
	}
}
