use std::net::{Ipv4Addr, Ipv6Addr};

use actix_web::HttpRequest;

use crate::{IC_HEADER_USER_ID, IC_HEADER_USER_IP};


pub fn get_user_id(request: &HttpRequest) -> Option<u64> {
	if let Some(header) = request.headers().get(IC_HEADER_USER_ID) {
		if let Ok(h) = header.to_str() {
			if let Ok(i) = h.parse::<u64>() {
				return Some(i);
			}
		}
	}
	return None;
}


/// Parses the "x-ic-user-ip" header value from requests, returning None if it doesn't exist.
/// If the user is connected via IPv4, this returns the IPv6 version of it.
pub fn get_user_ip(request: &HttpRequest) -> Option<Ipv6Addr> {
	// NOTE: MariaDB support for IPv4/6 sucks. It can't store/cast a IPv4
	// into an IPv6, until 11.3, or LTS 11.4 (May 2024). I know... SMH
	// Converting all IPv4's into IPv6's is technically a workaround to support
	// all MariaDB LTS versions. But I wouldn't fix this later. It's fine as is.
	if let Some(header) = request.headers().get(IC_HEADER_USER_IP) {
		if let Ok(h) = header.to_str() {
			// Try parsing as an IPV6 first
			if let Ok(i) = h.parse::<Ipv6Addr>() {
				return Some(i);
			} else {
				// Is it an IPv4?
				if let Ok(i) = h.parse::<Ipv4Addr>() {
					return Some(i.to_ipv6_compatible());
				}
			}
		}
	}
	return None;
}