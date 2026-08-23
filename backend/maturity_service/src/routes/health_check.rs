use actix_web::web::Data;

use ic_actix::{ICError, ICResult, AppStatePostgres};

/// The number of queued tasks allowed before the service will return /readyz errors
//pub const MAX_TASKS_IN_QUEUE:usize = 200; // I have NO IDEA if this is a decent value

//use tokio::runtime::Handle;
//use crate::ic_postgres::{AppStatePostgres};

pub async fn livez_status(pg: Data<AppStatePostgres>) -> ICResult<()> {
	if pg.health_check() {
		Ok(())
	} else {
		Err(ICError::HEAVY_LOAD)
	}
}

// Ok if the number of tasks is less than the const limit: MAX_TASKS_IN_QUEUE
pub async fn readyz_status() -> ICResult<()> {
	// TODO: This required a whole dependency, just for this & I'm not sure it's doing what it was supposed to anyway
	//if Handle::current().metrics().global_queue_depth() < MAX_TASKS_IN_QUEUE {
		Ok(())
	//} else {
	//	Err(ICError::HEAVY_LOAD)
	//}
}