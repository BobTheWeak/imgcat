
// At a minimum, we're just importing ICError
// Actix understands how to turn this into HTTP errors
mod ic_error;
mod error_actix;
pub use ic_error::{ICError, ICResult};

mod header_helpers;
pub use header_helpers::{get_bearer_jwt};

#[cfg(all(feature="redis", feature="postgres"))]
mod user_ban;
#[cfg(all(feature="redis", feature="postgres"))]
pub use user_ban::{check_temp_ban, check_perm_ban};

#[cfg(feature="redis")] mod app_state_redis;
#[cfg(feature="redis")] pub use app_state_redis::AppStateRedis;

#[cfg(feature="postgres")] mod app_state_postgres;
#[cfg(feature="postgres")] pub use app_state_postgres::AppStatePostgres;

#[cfg(feature="mariadb")] mod app_state_mariadb;
#[cfg(feature="mariadb")] pub use app_state_mariadb::AppStateMariaDB;

#[cfg(feature="cookies")] mod cookie;
#[cfg(feature="cookies")] pub use cookie::{ToCookie};