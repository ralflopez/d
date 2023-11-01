use std::collections::HashMap;

use super::{
    inventory_log::{self, InventoryLog, InventoryLogAction, InventoryLogForCreate},
    pageable::Pageable,
    user::get_user_ids,
    ModelManager,
};
use crate::model::error::Result;
use crate::{ctx::Ctx, model::inventory_log::InventoryLogActions};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;

// https://github.com/launchbadge/sqlx/issues/1004#issuecomment-854662251
#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(
    type_name = "inventory_transaction_action",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum InventoryTransactionAction {
    Sales,
    Deposit,
    SalesRollback,
    DepositRollback,
}

pub struct InventoryTransaction {
    id: i64,
    timestamp: DateTime<Utc>,
    action: InventoryTransactionAction,
    logs: Vec<InventoryLog>,
}

// region: Create
// region:      Shared
pub struct InventoryTransactionLogForCreate {
    pub quantity: i64,
    pub product_id: i64,
    pub price: f64,
    pub warehouse_id: i64,
}

pub struct InventoryTransactionForCreate {
    pub action: InventoryTransactionAction,
    pub logs: Vec<InventoryLogForCreate>,
}

impl InventoryTransactionForCreate {
    pub fn new(action: InventoryTransactionAction) -> Self {
        Self {
            action,
            logs: Vec::new(),
        }
    }

    pub fn add_log(&mut self, log: InventoryTransactionLogForCreate) -> () {
        let InventoryTransactionLogForCreate {
            price,
            product_id,
            quantity,
            warehouse_id,
        } = log;

        let new_log = match self.action {
            InventoryTransactionAction::Deposit => InventoryLogForCreate {
                quantity,
                product_id,
                action: InventoryLogAction::Incoming,
                price,
                warehouse_id,
            },
            InventoryTransactionAction::DepositRollback => InventoryLogForCreate {
                quantity,
                product_id,
                action: InventoryLogAction::Outgoing,
                price,
                warehouse_id,
            },
            InventoryTransactionAction::Sales => InventoryLogForCreate {
                quantity,
                product_id,
                action: InventoryLogAction::Outgoing,
                price,
                warehouse_id,
            },
            InventoryTransactionAction::SalesRollback => InventoryLogForCreate {
                quantity,
                product_id,
                action: InventoryLogAction::Incoming,
                price,
                warehouse_id,
            },
        };

        self.logs.push(new_log);
    }

    pub async fn save(self, ctx: &Ctx, mm: &ModelManager) -> Result<()> {
        create_inventory_transaction(ctx, mm, self).await
    }
}

async fn create_inventory_transaction(
    ctx: &Ctx,
    mm: &ModelManager,
    transaction_for_create: InventoryTransactionForCreate,
) -> Result<()> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let transaction = sqlx::query!(
        r#"INSERT INTO inventory_transactions (action, organization_id) 
        VALUES ($1, $2) 
        RETURNING id;"#,
        transaction_for_create.action as InventoryTransactionAction,
        organization_id
    )
    .fetch_one(db)
    .await?;

    let quantities: Vec<_> = transaction_for_create
        .logs
        .iter()
        .map(|l| l.quantity)
        .collect();

    let product_ids: Vec<_> = transaction_for_create
        .logs
        .iter()
        .map(|l| l.product_id)
        .collect();

    let actions: Vec<_> = transaction_for_create
        .logs
        .iter()
        .map(|l| l.action)
        .collect();

    let prices: Vec<_> = transaction_for_create
        .logs
        .iter()
        .map(|l| l.price)
        .collect();

    let organization_ids = vec![organization_id; transaction_for_create.logs.len()];
    let transaction_ids = vec![transaction.id; transaction_for_create.logs.len()];

    let warehouse_ids: Vec<_> = transaction_for_create
        .logs
        .iter()
        .map(|l| l.warehouse_id)
        .collect();

    sqlx::query!(
        r#"INSERT INTO inventory_logs (quantity, product_id, action, price, organization_id, warehouse_id, inventory_transaction_id)
        SELECT * FROM UNNEST($1::int8[], $2::int8[], $3::inventory_log_action[], $4::float8[], $5::int8[], $6::int8[], $7::int8[]);"#,
        &quantities,
        &product_ids,
        InventoryLogActions(&actions) as _,
        &prices,
        &organization_ids,
        &warehouse_ids,
        &transaction_ids
    ).execute_many(db)
    .await;

    Ok(())
}
// endregion:       Shared
// endregion: Create

// region: Read
#[derive(sqlx::FromRow)]
pub struct InventoryLogsWithTransactionAndProductForDbRow {
    inventory_transaction_id: i64,
    inventory_transaction_timestamp: DateTime<Utc>,
    inventory_transaction_action: InventoryTransactionAction,

    inventory_log_id: i64,
    inventory_log_quantity: i64,
    inventory_log_product_id: i64,
    inventory_log_action: InventoryLogAction,
    inventory_log_timestamp: DateTime<Utc>,
    inventory_log_price: BigDecimal,
    inventory_log_warehouse_id: i64,
    inventory_log_transaction_id: Option<i64>,

    product_sku: String,
    product_brand: String,
    product_name: String,
    product_display_name: String,
    product_description: String,
    product_price: BigDecimal,
}

// region:  Deposit
pub async fn get_all_deposit_transactions(
    ctx: &Ctx,
    mm: &ModelManager,
    pageable: Pageable,
) -> Result<Vec<InventoryTransaction>> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let deposits = sqlx::query_as!(
        InventoryLogsWithTransactionAndProductForDbRow,
        r#"SELECT
            it.id as inventory_transaction_id,
            it.timestamp as inventory_transaction_timestamp,
            it.action as "inventory_transaction_action: InventoryTransactionAction",
            
            il.id as inventory_log_id,
            il.quantity as inventory_log_quantity,
            il.product_id as inventory_log_product_id,
            il.action as "inventory_log_action: InventoryLogAction",
            il.timestamp as inventory_log_timestamp,
            il.price as inventory_log_price,
            il.warehouse_id as inventory_log_warehouse_id,
            il.inventory_transaction_id as inventory_log_transaction_id,

            p.sku as product_sku,
            p.brand as product_brand,
            p.name as product_name,
            p.display_name as product_display_name,
            p.description as product_description,
            p.price as product_price
        FROM inventory_logs il
        INNER JOIN inventory_transactions it
        ON il.inventory_transaction_id = it.id
        JOIN products p
        ON il.product_id = p.id
        WHERE
            it.organization_id = $1
        AND
            it.action = $2
        OFFSET $3
        LIMIT $4;"#,
        organization_id,
        InventoryTransactionAction::Deposit as InventoryTransactionAction,
        pageable.offset(),
        pageable.size()
    )
    .fetch_all(db)
    .await?;

    // fold
    let inventory_transactions: HashMap<i64, InventoryTransaction> =
        deposits.iter().fold(HashMap::new(), |mut acc, val| {
            acc.insert(
                val.inventory_transaction_id,
                InventoryTransaction {
                    id: val.inventory_transaction_id,
                    timestamp: val.inventory_transaction_timestamp,
                    action: val.inventory_transaction_action.to_owned(),
                    logs: Vec::new(),
                },
            );

            let key = val.inventory_log_transaction_id.unwrap_or(-1);
            let value = acc.get_mut(&key);
            value.map(|v| {
                v.logs.push(InventoryLog {
                    id: val.inventory_log_id,
                    quantity: val.inventory_log_quantity,
                    product_id: val.inventory_log_product_id,
                    product_display_name: val.product_display_name.to_owned(),
                    action: val.inventory_log_action,
                    timestamp: val.inventory_log_timestamp,
                    price: val.inventory_log_price.to_owned(),
                    warehouse_id: val.inventory_log_warehouse_id,
                    transaction_id: val.inventory_log_transaction_id,
                })
            });

            acc
        });

    let values: Vec<InventoryTransaction> = inventory_transactions.into_values().collect();
    Ok(values)
}
// endregion:   Deposit
// endregion: Read
