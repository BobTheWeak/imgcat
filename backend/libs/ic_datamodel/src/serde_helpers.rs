
pub mod bool_as_01 {
	use serde::{Serializer, Deserialize, Deserializer, de};

	pub fn serialize<S>(obj:&bool, s:S) -> Result<S::Ok, S::Error>
	where S:Serializer {
		match obj {
			false => s.serialize_u8(0),
			true => s.serialize_u8(1),
		}
	}

	pub fn deserialize<'a, D>(d:D) -> Result<bool, D::Error>
	where D:Deserializer<'a> {
		match u8::deserialize(d)? {
			0 => Ok(false),
			1 => Ok(true),
			bad => Err(de::Error::invalid_value(de::Unexpected::Unsigned(bad as u64), &"zero or one")),
		}
	}

}