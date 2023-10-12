use super::enums::InventoryLogAction;
use super::inventory_transaction::{DepositForCreateItem, SalesForCreateItem};
use super::{user, ModelManager};
use crate::ctx::Ctx;
use crate::model::common::RowWithId;
use crate::model::enums::InventoryLogActions;
use crate::model::error::Result;

// region: Structs
pub struct InventoryLogForCreate {
    pub quantity: i64,
    pub product_id: i64,
    pub action: InventoryLogAction,
    pub price: f64,
    pub warehouse_id: i64,
}

impl From<DepositForCreateItem> for InventoryLogForCreate {
    fn from(deposit_for_create_item: DepositForCreateItem) -> Self {
        Self {
            quantity: deposit_for_create_item.quantity,
            product_id: deposit_for_create_item.product_id,
            action: InventoryLogAction::Incoming,
            price: deposit_for_create_item.price,
            warehouse_id: deposit_for_create_item.warehouse_id,
        }
    }
}

impl From<SalesForCreateItem> for InventoryLogForCreate {
    fn from(sell_for_create_item: SalesForCreateItem) -> Self {
        Self {
            quantity: sell_for_create_item.quantity,
            product_id: sell_for_create_item.product_id,
            action: InventoryLogAction::Outgoing,
            price: sell_for_create_item.price,
            warehouse_id: sell_for_create_item.warehouse_id,
        }
    }
}

// endregion: Structs

// region: Methods
pub async fn add_logs(
    ctx: &Ctx,
    mm: &ModelManager,
    inventory_log_for_create: Vec<InventoryLogForCreate>,
) -> Result<Vec<i64>> {
    let db = mm.db();

    let (_, organization_id) = user::get_user_ids(ctx, mm).await?;

    let quantities: Vec<_> = inventory_log_for_create
        .iter()
        .map(|i| i.quantity)
        .collect();
    let product_ids: Vec<_> = inventory_log_for_create
        .iter()
        .map(|i| i.product_id)
        .collect();
    let actions: Vec<_> = inventory_log_for_create.iter().map(|i| i.action).collect();
    let prices: Vec<_> = inventory_log_for_create.iter().map(|i| i.price).collect();
    let organization_ids: Vec<_> = inventory_log_for_create
        .iter()
        .map(|_i| organization_id.clone())
        .collect();
    let warehouse_ids: Vec<_> = inventory_log_for_create
        .iter()
        .map(|i| i.warehouse_id)
        .collect();

    let result_ids = sqlx::query_as!(
        RowWithId,
        r#"INSERT INTO inventory_logs (quantity, product_id, action, price, organization_id, warehouse_id)
        SELECT * FROM UNNEST($1::int8[], $2::int8[], $3::inventory_log_action[], $4::float8[], $5::int8[], $6::int8[])
        RETURNING id;"#,
        &quantities,
        &product_ids,
        InventoryLogActions(&actions) as _,
        &prices,
        &organization_ids,
        &warehouse_ids
    )
    .fetch_all(db)
    .await?;

    Ok(result_ids.iter().map(|r| r.id).collect())
}
// endregion: Methods
