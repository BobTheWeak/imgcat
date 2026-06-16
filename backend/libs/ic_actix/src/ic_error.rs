use std::fmt;

pub type ICResult<T> = Result<T, ICError>;

#[derive(Debug)]
pub struct ICError {
	pub(crate) status: u16,
	pub(crate) message: &'static str
}

impl ICError {
	// Anything not standardized, or call-specific should use this
	pub const fn new(status: u16, message: &'static str) -> Self {
		Self {status, message}
	}

	/// error() is used for standard application errors
	pub const fn error(message: &'static str) -> Self {
		Self {status:400, message}
	}

	/// panic() is used for service outages
	pub const fn panic(message: &'static str) -> Self {
		Self {status:500, message}
	}

	// Authentication errors (ie. the Bearer token is missing or expired)
	pub const HEADER_MISSING:ICError = ICError::new(401, "Auth header");
	pub const HEADER_VALIDATION:ICError = ICError::new(403, "Auth validation");

	// User bans & guards
	pub const BAN_IP:ICError = ICError::new(418, "Temporary IP ban");
	pub const BAN_TEMP:ICError = ICError::new(418, "Temporary user ban");
	pub const BAN_PERM:ICError = ICError::new(418, "Permanent user ban");
	pub const RATE_LIMIT:ICError = ICError::new(429, "Rate limit");
	
	// Common application errors
	// If for some reason we can't translate a link to an ID
	pub const LINK_NOT_FOUND:ICError = ICError::new(400, "Link not found");


	// Service connection errors (ie. Postgres isn't available, check your ENV VARs)
	pub const POSTGRES_CONN:ICError = ICError::new(503, "Postgres connection");
	pub const REDIS_CONN:ICError = ICError::new(503, "Redis connection");
	pub const MARIADB_CONN:ICError = ICError::new(503, "MariaDB connection");

	// Pool errors - These are issues with Actix/Tokio/mutex when fetching a new conn
	pub const POOL_ERROR:ICError = ICError::new(503, "Connection pool");

	// Health-check failed, the service is overloaded
	pub const HEAVY_LOAD:ICError = ICError::new(503, "Too many requests");
}


impl fmt::Display for ICError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "{}", self.message)
	}
}
