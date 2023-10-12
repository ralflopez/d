pub mod category;
mod common;
pub mod enums;
mod error;
pub mod inventory_log;
pub mod inventory_transaction;
pub mod organization;
pub mod pageable;
pub mod permissions;
pub mod products;
mod store;
pub mod user;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
