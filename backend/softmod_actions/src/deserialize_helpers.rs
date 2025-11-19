use serde::{de, Deserialize};

pub fn bool_from_int<'a, D>(d:D) -> Result<bool, D::Error> 
where D:de::Deserializer<'a> {
	// TODO: Should recognize "/url?value1&value2" as both set/true
	// It can save a few bytes, but it may complicate logic/testing
	match u8::deserialize(d)? {
		0 => Ok(false),
		1 => Ok(true),
		bad => Err(de::Error::invalid_value(de::Unexpected::Unsigned(bad as u64), &"zero or one")),
	}
}