
use actix_web::web::{Data, Json};
use postgres_types::{Type};

/*use ic_jwt::{validate_with_defaults};*/
use ic_actix::{AppStatePostgres, /*get_bearer_jwt,*/ ICResult, ICError};
use ic_datamodel::{PostMaturity};


pub async fn scores_by_ids(
		postgres: Data<AppStatePostgres>,
		body_data: Json<Vec<i64>>
		//request: HttpRequest
	) -> ICResult<Json<Vec<PostMaturity>>> {
	
	// TODO: Do we need this? Can we do this? This will be called as a service-to-service call, NOT by a user.
	// TODO: We need to create a new service-to-service JWT... This is a whole big new thing... Bleh.
	//let jwt_string = get_bearer_jwt(&request)?;
	//if !validate_with_defaults(jwt_string) {
	//	return Err(ICError::HEADER_VALIDATION);
	//}

	let conn = postgres.get_conn().await?;

	// TODO: This is done as a block DL for debugging/documentation purposes, but we should do streaming.
	// I spent a little time on it, but streaming is (shocked face) both tricky, awful, and without proper documentation
	// Postgres uses futures, JSON is "chunky" but can do write-as-strings, and Actix requires Bytes as a zero-copy buffer
	// There isn't a simple, Rusty way of doing this. We have to juggle everything manually.
	let result = match conn.query_typed(
		"SELECT * FROM Results.GetMaturityScoresByPostIds($1)",
		&[ (&body_data.to_vec(), Type::INT8_ARRAY) ]
	).await {
		Ok(data) => {
			data.iter().map(|row|{ row.into() }).collect()
		},
		Err(e) => {
			println!("{}", e);
			return Err(ICError::POSTGRES_CONN);
		}
	};

	return Ok(Json(result));
}