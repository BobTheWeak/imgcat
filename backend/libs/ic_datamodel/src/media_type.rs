use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
	RasterImage = 1,
	VectorImage = 2,
	Animation = 3,
	Video = 4,
}

impl MediaType {
	// This is the official list of file types we allow
	pub const RASTER_TYPES:[&str; 3] =    ["image/png", "image/jpeg", "image/webp"];
	pub const VECTOR_TYPES:[&str; 1] =    ["image/svg+xml"];
	pub const ANIMATION_TYPES:[&str; 2] = ["image/gif", "image/apng"];
	pub const VIDEO_TYPES:[&str; 2] =     ["video/mp4", "video/webm"];

	// Is it ugly? Yes. But it works.
	pub const ALL_TYPES:[&str; 8] = [
		Self::RASTER_TYPES[0], Self::RASTER_TYPES[1], Self::RASTER_TYPES[2],
		Self::VECTOR_TYPES[0],
		Self::ANIMATION_TYPES[0], Self::ANIMATION_TYPES[1],
		Self::VIDEO_TYPES[0], Self::VIDEO_TYPES[1],
	];
}

/***** MariaDB Feature *****/

#[cfg(feature="mariadb")]
use mysql_common::value::{Value, convert::FromValue, convert::FromValueError};

#[cfg(feature="mariadb")]
impl FromValue for MediaType {
	type Intermediate = MediaType;
}

#[cfg(feature="mariadb")]
impl TryFrom<Value> for MediaType {
	type Error = FromValueError;

	fn try_from(v: Value) -> Result<Self, Self::Error> {
		let v:u8 = match v {
			Value::Int(v) => v as u8,
			Value::UInt(v) => v as u8,
			v => return Err(FromValueError(v)),
		};
		return match v {
			1 => Ok(Self::RasterImage),
			2 => Ok(Self::VectorImage),
			3 => Ok(Self::Animation),
			4 => Ok(Self::Video),
			v => Err(FromValueError(Value::UInt(v as u64))),
		};
	}
}

/***** PostgreSQL Feature *****/

/*
#[cfg(feature="postgres")]
use postgres_types::{FromSql};

#[cfg(feature="postgres")]
impl FromSql for MediaType {

}
*/