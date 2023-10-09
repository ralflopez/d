use super::{
    common::RowWithId,
    enums::InventoryTransactionAction,
    inventory_log::{add_logs, InventoryLogForCreate},
    pageable::Pageable,
    ModelManager, Result,
};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;

// region: Structs
#[derive(Debug)]
pub struct DepositLogForDbResult {
    pub id: i64,
    pub purchase_price: BigDecimal,
    pub warehouse_id: i64,
    // https://stackoverflow.com/questions/73626570/trait-bound-chronodatetimeutc-fromsqldieselsql-typesnullablediesel
    pub timestamp: Option<DateTime<Utc>>,
    pub action: InventoryTransactionAction,
    pub sku: String,
    pub display_name: String,
}

pub struct DepositForCreateItem {
    pub quantity: i64,
    pub product_id: i64,
    pub price: f64,
    pub warehouse_id: i64,
}

pub struct DepositForCreate {
    pub items: Vec<DepositForCreateItem>,
}

pub struct SalesForCreateItem {
    pub quantity: i64,
    pub product_id: i64,
    pub price: f64,
    pub warehouse_id: i64,
}

pub struct SalesForCreate {
    pub items: Vec<SalesForCreateItem>,
}
// endregion: Structs

// region: Methods
pub async fn get_deposit_logs(
    mm: &ModelManager,
    pageable: Pageable,
) -> Result<Vec<DepositLogForDbResult>> {
    let db = mm.db();
    // TODO: get organization id from context
    let organization_id: i64 = 1;
    let offset = pageable.offset();
    let page_size = pageable.size();

    let logs = sqlx::query_as!(
        DepositLogForDbResult,
        r#"SELECT
            it.id as id,
            il.price as purchase_price,
            il.warehouse_id,
            it.timestamp as timestamp,
            it.action as "action: InventoryTransactionAction",
            p.sku as sku,
            p.display_name as display_name
        FROM inventory_transaction_items iti
        INNER JOIN inventory_logs il ON iti.inventory_log_id = il.id
        INNER JOIN inventory_transactions it ON iti.inventory_transaction_id = it.id
        INNER JOIN products p ON il.product_id = p.id
        WHERE it.organization_id = $1
        AND it.action = 'DEPOSIT'
        OFFSET $2 LIMIT $3;"#,
        organization_id,
        offset,
        page_size
    )
    .fetch_all(db)
    .await?;

    Ok(logs)
}

pub async fn add_deposit(mm: &ModelManager, deposit_for_create: DepositForCreate) -> Result<()> {
    let db = mm.db();
    // TODO: get organization id from context
    let organization_id: i64 = 1;

    let items = deposit_for_create
        .items
        .into_iter()
        .map(InventoryLogForCreate::from)
        .collect();
    let tx = db.begin().await?;
    let log_ids = add_logs(mm, items).await?;

    let transaction = sqlx::query_as!(
        RowWithId,
        r#"INSERT INTO inventory_transactions (organization_id, action) VALUES ($1, $2) RETURNING id;"#,
        organization_id,
        InventoryTransactionAction::Deposit as InventoryTransactionAction
    )
    .fetch_one(db)
    .await?;

    let transaction_ids_for_items: Vec<_> =
        log_ids.iter().map(|_l| transaction.id.clone()).collect();
    sqlx::query!(
        r#"INSERT INTO inventory_transaction_items (inventory_transaction_id, inventory_log_id)
        SELECT * FROM UNNEST($1::int8[], $2::int8[]);"#,
        &log_ids,
        &transaction_ids_for_items
    )
    .execute(db)
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn add_sales(mm: &ModelManager, sell_for_create: SalesForCreate) -> Result<()> {
    let db = mm.db();
    // TODO: get organization id from context
    let organization_id: i64 = 1;

    let items: Vec<InventoryLogForCreate> = sell_for_create
        .items
        .into_iter()
        .map(InventoryLogForCreate::from)
        .collect();

    let tx = db.begin().await?;
    let log_ids = add_logs(mm, items).await?;

    let transaction = sqlx::query_as!(
        RowWithId,
        r#"INSERT INTO inventory_transactions (organization_id, action) VALUES ($1, $2) RETURNING id;"#,
        organization_id,
        InventoryTransactionAction::Sales as InventoryTransactionAction
    )
    .fetch_one(db)
    .await?;

    let transaction_ids_for_items: Vec<_> =
        log_ids.iter().map(|_l| transaction.id.clone()).collect();
    sqlx::query!(
        r#"INSERT INTO inventory_transaction_items (inventory_transaction_id, inventory_log_id)
        SELECT * FROM UNNEST($1::int8[], $2::int8[]);"#,
        &log_ids,
        &transaction_ids_for_items
    )
    .execute(db)
    .await?;

    tx.commit().await?;
    Ok(())
}
// endregion: Methods
