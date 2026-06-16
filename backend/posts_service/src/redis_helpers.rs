use redis::{Connection, Cmd};

use ic_actix::{ICResult, ICError};

pub(crate) fn get_post_views(conn:&mut Connection, link:&str) -> ICResult<Option<u32>> {
	let Ok(value) = Cmd::new()
		.arg("GET")
		.arg(format!("pv:{}", link)) // ie. Post Views
		.query::<Option<u32>>(conn) else { return Err(ICError::REDIS_CONN) };

	Ok(value)
}

pub(crate) fn set_post_views(conn:&mut Connection, link:&str, value:u32) -> ICResult<()> {
	let Ok(_) = Cmd::new()
		.arg("SET")
		.arg(format!("pv:{}", link)) // ie. Post Views
		.arg(value)
		.query::<Option<u32>>(conn) else { return Err(ICError::REDIS_CONN) };

	Ok(())
}

pub(crate) fn get_post_votes(conn:&mut Connection, link:&str) -> ICResult<Option<u32>> {
	let Ok(value) = Cmd::new()
		.arg("GET")
		.arg(format!("pp:{}", link)) // ie. Post Points
		.query::<Option<u32>>(conn) else { return Err(ICError::REDIS_CONN) };

	Ok(value)
}

pub(crate) fn set_post_votes(conn:&mut Connection, link:&str, value:u32) -> ICResult<()> {
	let Ok(_) = Cmd::new()
		.arg("SET")
		.arg(format!("pp:{}", link)) // ie. Post Points
		.arg(value)
		.query::<Option<u32>>(conn) else { return Err(ICError::REDIS_CONN) };

	Ok(())
}


//pub(crate) fn get_post_id_by_link(conn:&mut Connection, link:&str) -> ICResult<Option<i64>> {
//	let Ok(value) = Cmd::new()
//		.arg("GET")
//		// NOTE: li = link-to-id
//		.arg(format!("li:{}", link))
//		.query::<Option<i64>>(conn) else { return Err(ICError::REDIS_CONN) };
//
//	Ok(value)
//}


//pub(crate) fn set_post_id_by_link(conn:&mut Connection, link:&str, value:i64) -> ICResult<()> {
//	let Ok(_) = Cmd::new()
//		.arg("SET")
//		// NOTE: li = link-to-id
//		.arg(format!("li:{}", link))
//		.arg(value)
//		.query::<Option<i64>>(conn) else { return Err(ICError::REDIS_CONN) };
//
//	Ok(())
//}
