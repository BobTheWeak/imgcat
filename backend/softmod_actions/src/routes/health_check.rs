/// The number of queued tasks allowed before the service will return /readyz errors
pub const MAX_TASKS_IN_QUEUE:usize = 200; // I have NO IDEA if this is a decent value

use tokio::runtime::Handle;
use actix_web::{get, HttpResponse};
use crate::conn_helpers::connect;

#[get("/livez")]
/// Ok if we can fetch a connection from the pool
async fn livez_status() -> HttpResponse {
	if connect().await.is_ok() {
		return HttpResponse::Ok().finish(); // 200
	} else {
		return HttpResponse::ServiceUnavailable().finish(); // 503
	}
}

#[get("/readyz")]
// Ok if the number of tasks is less than the const limit: MAX_TASKS_IN_QUEUE
async fn readyz_status() -> HttpResponse {
	if Handle::current().metrics().global_queue_depth() < MAX_TASKS_IN_QUEUE {
		return HttpResponse::Ok().finish();	
	} else {
		return HttpResponse::ServiceUnavailable().finish(); // 503
	}
}