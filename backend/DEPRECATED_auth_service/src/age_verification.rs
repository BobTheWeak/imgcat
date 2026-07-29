use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AgeVerification {
	pub country: String,
	pub state: Option<String>,
	pub age: u16,
}

impl AgeVerification {
	pub fn new(country:&str, state:Option<&str>, age:u16) -> Self {
		Self {
			country: country.to_string(),
			state: state.and_then(|x| Some(x.to_string())),
			age,
		}
	}
}