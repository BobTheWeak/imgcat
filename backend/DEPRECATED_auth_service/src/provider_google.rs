use std::time::{SystemTime, Duration, UNIX_EPOCH};
use serde_json::{Value};
use openidconnect::reqwest;


fn calc_age(y:u16, m:u8, d:u8) -> u16 {
	let y:u64 = y.into();
	let m:u64 = m.into();
	let d:u64 = d.into();

	let age_dur:Duration = if y >= 1970 {
		// after the epoch, normal math
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
			- Duration::new((y-1970) * 31_556_952_u64, 0)
	} else {
		// before the epoch, reverse it
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
			+ Duration::new((1970-y) * 31_556_952_u64, 0)
	}
	// Either way, subtract months/days
	- Duration::new(m * 2_629_746_u64 + d * 86_400_u64, 0)
	// Add 1 day grace for timezones
	+ Duration::from_secs(24 * 60 * 60);

	return (age_dur.as_secs() / 31_556_952_u64).try_into().unwrap_or(0_u16);
}


pub async fn getage(auth_token: &str) -> Option<u16> {

	// Call the Google API with our Bearer token
	let age_request = reqwest::Client::new();
	let res = age_request
		.get("https://people.googleapis.com/v1/people/me?personFields=birthdays")
		.header("Authorization", "Bearer ".to_owned() + auth_token)
		.send()
		.await;

	// Extract the response as JSON (it should always return something)
	let json = {
		let Ok(res) = res else {return None};
		let Ok(body) = res.text().await else {return None};
		let Ok(json) = serde_json::from_str::<Value>(&body) else {return None};
		json
	};
	
	// Just grab /birthdays, ignore everything else (metadata & etag)
	let Some(json) = json.get("birthdays").and_then(|x| x.as_array()) else {
		// They declined the permission
		return Some(0);
	};

	// A Google b-day needs to be reconstructed, but date blocks aren't
	// guaranteed to have each part. So loop through them all. Last wins.
	let mut y:u16 = 0;
	let mut m:u8 = 0;
	let mut d:u8 = 0;

	for entry in json.iter() {
		// NOTE: ?-chaining breaks the for-loop, not the if-some.
		if let Some(date) = entry.get("date").and_then(|x| x.as_object()) {
			date.get("day")
				.and_then(|x| x.as_number())
				.and_then(|x| x.as_u64()) // JSON Number can't just parse to u8
				.and_then(|x| Some(d = x as u8));
			date.get("month")
				.and_then(|x| x.as_number())
				.and_then(|x| x.as_u64())
				.and_then(|x| Some(m = x as u8));
			date.get("year")
				.and_then(|x| x.as_number())
				.and_then(|x| x.as_u64())
				.and_then(|x| Some(y = x as u16));
		}
	}

	//println!("Birthday: {}-{}-{}", y, m, d);

	return Some(calc_age(y,m,d));
}