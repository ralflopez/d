use sqlx::postgres::PgTypeInfo;

#[derive(sqlx::Type, Debug)]
#[sqlx(
    type_name = "inventory_transaction_action",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum InventoryTransactionAction {
    Deposit,
    Sales,
}

// https://github.com/launchbadge/sqlx/issues/1004#issuecomment-854662251
#[derive(sqlx::Type, Debug, Clone, Copy)]
#[sqlx(
    type_name = "inventory_log_action",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum InventoryLogAction {
    Incoming,
    Outgoing,
    IncomingRollback,
    OutgoingRollback,
}

// https://github.com/launchbadge/sqlx/issues/298#issuecomment-908511000
#[derive(sqlx::Encode)]
pub struct InventoryLogActions<'a>(pub &'a [InventoryLogAction]);

impl sqlx::Type<sqlx::Postgres> for InventoryLogActions<'_> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_inventory_log_action")
    }
}
