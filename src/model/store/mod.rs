mod error;

pub use self::error::{Error, Result};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(10)
        // TODO: fetch from env
        .connect("postgres://test:test@localhost:5432/distrupify_test")
        .await
        .map_err(|e| Error::FailToCreatePool(e.to_string()))
}
