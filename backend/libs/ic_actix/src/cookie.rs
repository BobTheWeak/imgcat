use actix_web::cookie::{Cookie, SameSite, time::Duration};

use ic_jwt::{AuthJwt, EncodeJwt};
#[cfg(feature = "refresh_jwt")]
use ic_jwt::RefreshJwt;
#[cfg(feature = "signup_jwt")]
use ic_jwt::SignupJwt;


pub trait ToCookie {
	fn to_cookie(&self) -> Cookie
		where Self:EncodeJwt;
}


impl ToCookie for AuthJwt {
	fn to_cookie(self:&Self) -> Cookie {
		Cookie::build("ic_auth", self.encode_with_defaults().unwrap())
			.path("/")
			.secure(true)
			.http_only(true)
			.same_site(SameSite::Lax) // TODO: Strict isn't working
			.max_age(Duration::minutes(5))
			.finish()
	}
}

#[cfg(feature = "refresh_jwt")]
impl ToCookie for RefreshJwt {
	fn to_cookie(self:&Self) -> Cookie {
		Cookie::build("ic_refresh", self.encode_with_defaults().unwrap())
			.path("/")
			.secure(true)
			.http_only(true)
			.same_site(SameSite::Lax) // TODO: Strict isn't working
			.max_age(Duration::weeks(2))
			.finish()
	}
}

#[cfg(feature = "signup_jwt")]
impl ToCookie for SignupJwt {
	fn to_cookie(self:&Self) -> Cookie {
		Cookie::build("ic_signup", self.encode_with_defaults().unwrap())
			.path("/") // Should be just "/signup", but it breaks remote functions
			.secure(true)
			.http_only(true)
			.same_site(SameSite::Lax) // TODO: Strict isn't working
			.max_age(Duration::minutes(15))
			.finish()
	}
}

