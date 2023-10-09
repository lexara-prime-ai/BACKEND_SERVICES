use diesel::PgConnection;
use rocket_sync_db_pools;

pub mod rustaceans;
pub mod crates;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);
