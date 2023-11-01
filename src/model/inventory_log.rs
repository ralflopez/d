use super::{pageable::Pageable, ModelManager};
use crate::model::Result;
use crate::{ctx::Ctx, model::user::get_user_ids};
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgTypeInfo, types::BigDecimal};

// region: Structs
// https://github.com/launchbadge/sqlx/issues/1004#issuecomment-854662251
#[derive(sqlx::Type, Debug, Clone, Copy)]
#[sqlx(
    type_name = "inventory_log_action",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum InventoryLogAction {
    Incoming,
    Outgoing,
}

// https://github.com/launchbadge/sqlx/issues/298#issuecomment-908511000
#[derive(sqlx::Encode)]
pub struct InventoryLogActions<'a>(pub &'a [InventoryLogAction]);

impl sqlx::Type<sqlx::Postgres> for InventoryLogActions<'_> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_inventory_log_action")
    }
}

#[derive(sqlx::FromRow)]
pub struct InventoryLog {
    pub id: i64,
    pub quantity: i64,
    pub product_id: i64,
    pub product_display_name: String,
    pub action: InventoryLogAction,
    pub timestamp: DateTime<Utc>,
    pub price: BigDecimal,
    pub warehouse_id: i64,
    pub transaction_id: Option<i64>,
}

// region: Create
pub struct InventoryLogForCreate {
    pub quantity: i64,
    pub product_id: i64,
    pub action: InventoryLogAction,
    pub price: f64,
    pub warehouse_id: i64,
}
// endregion: Create

// region: Read
pub async fn get_logs(
    ctx: &Ctx,
    mm: &ModelManager,
    pageable: Pageable,
) -> Result<Vec<InventoryLog>> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let logs: Vec<InventoryLog> = sqlx::query_as!(
        InventoryLog,
        r#"SELECT
            il.id,
            il.quantity,
            il.product_id,
            p.display_name as product_display_name,
            il.action as "action: InventoryLogAction",
            il.timestamp,
            il.price,
            il.warehouse_id,
            il.inventory_transaction_id as transaction_id
        FROM inventory_logs il
        JOIN products p
        ON p.id = il.product_id
        WHERE 
        il.organization_id = $1 
        OFFSET $2 
        LIMIT $3"#,
        organization_id,
        pageable.offset(),
        pageable.size()
    )
    .fetch_all(db)
    .await?;

    Ok(logs.into_iter().map(|l| l.into()).collect())
}
// region: Read

// impl From<DepositForCreateItem> for InventoryLogForCreateOld {
//     fn from(deposit_for_create_item: DepositForCreateItem) -> Self {
//         Self {
//             quantity: deposit_for_create_item.quantity,
//             product_id: deposit_for_create_item.product_id,
//             action: InventoryLogAction::Incoming,
//             price: deposit_for_create_item.price,
//             warehouse_id: deposit_for_create_item.warehouse_id,
//         }
//     }
// }

// impl From<SalesForCreateItem> for InventoryLogForCreateOld {
//     fn from(sell_for_create_item: SalesForCreateItem) -> Self {
//         Self {
//             quantity: sell_for_create_item.quantity,
//             product_id: sell_for_create_item.product_id,
//             action: InventoryLogAction::Outgoing,
//             price: sell_for_create_item.price,
//             warehouse_id: sell_for_create_item.warehouse_id,
//         }
//     }
// }

// endregion: Structs

// region: Methods
// pub async fn add_logs(
//     ctx: &Ctx,
//     mm: &ModelManager,
//     inventory_log_for_create: Vec<InventoryLogForCreateOld>,
// ) -> Result<Vec<i64>> {
//     let db = mm.db();

//     let (_, organization_id) = user::get_user_ids(ctx, mm).await?;

//     let quantities: Vec<_> = inventory_log_for_create
//         .iter()
//         .map(|i| i.quantity)
//         .collect();
//     let product_ids: Vec<_> = inventory_log_for_create
//         .iter()
//         .map(|i| i.product_id)
//         .collect();
//     let actions: Vec<_> = inventory_log_for_create.iter().map(|i| i.action).collect();
//     let prices: Vec<_> = inventory_log_for_create.iter().map(|i| i.price).collect();
//     let organization_ids: Vec<_> = inventory_log_for_create
//         .iter()
//         .map(|_i| organization_id)
//         .collect();
//     let warehouse_ids: Vec<_> = inventory_log_for_create
//         .iter()
//         .map(|i| i.warehouse_id)
//         .collect();

//     let result_ids = sqlx::query_as!(
//         RowWithId,
//         r#"INSERT INTO inventory_logs (quantity, product_id, action, price, organization_id, warehouse_id)
//         SELECT * FROM UNNEST($1::int8[], $2::int8[], $3::inventory_log_action[], $4::float8[], $5::int8[], $6::int8[])
//         RETURNING id;"#,
//         &quantities,
//         &product_ids,
//         InventoryLogActions(&actions) as _,
//         &prices,
//         &organization_ids,
//         &warehouse_ids
//     )
//     .fetch_all(db)
//     .await?;

//     Ok(result_ids.iter().map(|r| r.id).collect())
// }
// endregion: Methods
