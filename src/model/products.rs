use sqlx::types::BigDecimal;

use crate::ctx::Ctx;

use super::{user, ModelManager, Result};

// region: Structs
#[derive(Debug)]
pub struct ProductStockLevelForDbResult {
    pub product_id: i64,
    pub sku: String,
    pub brand: String,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub quantity: Option<i64>,
}

pub struct StockLevelForDbResult {
    pub quantity: Option<i64>,
}
// endregion: Structs

// region: Methods
pub async fn get_all_stock_levels(
    ctx: &Ctx,
    mm: &ModelManager,
) -> Result<Vec<ProductStockLevelForDbResult>> {
    let db = mm.db();
    let (_, organization_id) = user::get_user_ids(ctx, mm).await?;

    let products = sqlx::query_as!(
        ProductStockLevelForDbResult,
        r#"SELECT
          product_id,
          p.sku,
          p.brand,
          p.name,
          p.description,
          p.price,
          SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END) as quantity
        FROM inventory_logs il
        JOIN products p
        ON p.id = il.product_id
        WHERE il.organization_id = $1
        GROUP BY product_id, p.sku, p.brand, p.name, p.description, p.price;"#,
        organization_id
    )
    .fetch_all(db)
    .await?;

    Ok(products)
}

pub async fn get_stock_level(ctx: &Ctx, mm: &ModelManager, product_id: i64) -> Result<i64> {
    let db = mm.db();
    let (_, organization_id) = user::get_user_ids(ctx, mm).await?;

    let stock_level = sqlx::query_as!(
        StockLevelForDbResult,
        r#"SELECT
          SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END) as quantity
        FROM inventory_logs il
        JOIN products p
        ON p.id = il.product_id
        WHERE il.organization_id = $1
        GROUP BY product_id
        HAVING product_id = $2;"#,
        organization_id,
        product_id
    )
    .fetch_one(db)
    .await?;

    Ok(match stock_level.quantity {
        Some(s) => s,
        None => 0,
    })
}
// endregion: Methods
