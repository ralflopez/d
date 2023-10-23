use super::{
    user::{self, get_user_ids},
    ModelManager, Result,
};
use crate::ctx::Ctx;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

// region: Structs
#[derive(Debug)]
pub struct ProductStockLevelForDbResult {
    pub id: i64,
    pub sku: String,
    pub brand: String,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub quantity: Option<i64>,
}

#[derive(Debug)]
pub struct ProductWithStockLevel {
    pub id: i64,
    pub sku: String,
    pub brand: String,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub quantity: i64,
}

impl From<ProductStockLevelForDbResult> for ProductWithStockLevel {
    fn from(value: ProductStockLevelForDbResult) -> Self {
        Self {
            id: value.id,
            sku: value.sku,
            brand: value.brand,
            name: value.name,
            description: value.description,
            price: value.price,
            quantity: value.quantity.unwrap_or(0),
        }
    }
}
// endregion: Structs

// region: Methods
pub async fn get_all_products_with_stock_levels(
    ctx: &Ctx,
    mm: &ModelManager,
) -> Result<Vec<ProductWithStockLevel>> {
    let db = mm.db();
    let (_, organization_id) = user::get_user_ids(ctx, mm).await?;

    let products = sqlx::query_as!(
        ProductStockLevelForDbResult,
        r#"SELECT
            p.id,
            p.sku,
            p.brand,
            p.name,
            p.description,
            p.price,
            COALESCE(SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END), 0) as quantity
        FROM products p
        LEFT JOIN inventory_logs il
        ON p.id = il.product_id
        WHERE p.organization_id = $1
        GROUP BY p.id, p.sku, p.brand, p.name, p.description, p.display_name, p.price
        ORDER BY p.display_name;"#,
        organization_id
    )
    .fetch_all(db)
    .await?;

    Ok(products.into_iter().map(|p| p.into()).collect())
}

pub async fn get_product_with_stock_level(
    ctx: &Ctx,
    mm: &ModelManager,
    product_id: i64,
) -> Result<Option<ProductWithStockLevel>> {
    let db = mm.db();
    let (_, organization_id) = user::get_user_ids(ctx, mm).await?;

    let product = sqlx::query_as!(
        ProductStockLevelForDbResult,
        r#"SELECT
            p.id,
            p.sku,
            p.brand,
            p.name,
            p.description,
            p.price,
            COALESCE(SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END), 0) as quantity
        FROM products p
        LEFT JOIN inventory_logs il
        ON p.id = il.product_id
        WHERE p.id = $1 
        AND p.organization_id = $2
        GROUP BY p.id, p.sku, p.brand, p.name, p.description, p.price;"#,
        product_id,
        organization_id
    )
    .fetch_optional(db)
    .await?;

    Ok(product.map(|p| p.into()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductForCreate {
    pub sku: String,
    pub brand: String,
    pub name: String,
    pub description: String,
    pub price: i64,
}
pub async fn create_product(
    ctx: &Ctx,
    mm: &ModelManager,
    product_for_create: ProductForCreate,
) -> Result<()> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let ProductForCreate {
        sku,
        brand,
        name,
        description,
        price,
    } = product_for_create;

    sqlx::query!(
        r#"INSERT INTO products 
        (sku, brand, name, description, display_name, price, organization_id) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        sku,
        brand,
        name,
        description,
        format!("{} {} {}", brand, name, description),
        BigDecimal::from(price),
        organization_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete_product(ctx: &Ctx, mm: &ModelManager, product_id: i64) -> Result<()> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    sqlx::query!(
        "DELETE FROM products WHERE id = $1 AND organization_id = $2;",
        product_id,
        organization_id
    )
    .execute(db)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductForUpdate {
    pub sku: String,
    pub brand: String,
    pub name: String,
    pub description: String,
    pub price: i64,
}
pub async fn update_product(
    ctx: &Ctx,
    mm: &ModelManager,
    id: i64,
    product_for_update: ProductForUpdate,
) -> Result<()> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let ProductForUpdate {
        sku,
        brand,
        name,
        description,
        price,
    } = product_for_update;

    sqlx::query!(
        r#"UPDATE products 
        SET 
            sku = $1,
            brand = $2,
            name = $3,
            description = $4,
            price = $5
        WHERE id = $6
        AND organization_id = $7;"#,
        sku,
        brand,
        name,
        description,
        BigDecimal::from(price),
        id,
        organization_id
    )
    .execute(db)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProductSearchByType {
    #[serde(rename = "sku")]
    Sku,
    #[serde(rename = "full_name")]
    FullName,
    #[serde(rename = "brand")]
    Brand,
    #[serde(rename = "name")]
    Name,
}
#[derive(Debug, Deserialize)]
pub struct ProductForSearch {
    pub search: String,
    pub by: ProductSearchByType,
}
pub async fn search_products(
    ctx: &Ctx,
    mm: &ModelManager,
    product_for_search: ProductForSearch,
) -> Result<Vec<ProductWithStockLevel>> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let products = match product_for_search.by {
        ProductSearchByType::Sku => {
            sqlx::query_as!(
                ProductStockLevelForDbResult,
                r#"SELECT
                    p.id,
                    p.sku,
                    p.brand,
                    p.name,
                    p.description,
                    p.price,
                    COALESCE(SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END), 0) as quantity
                FROM products p
                LEFT JOIN inventory_logs il
                ON p.id = il.product_id
                WHERE p.organization_id = $1
                AND p.sku LIKE $2
                GROUP BY p.id, p.sku, p.brand, p.name, p.description, p.price
                ORDER BY p.sku;"#,
                organization_id,
                format!("{}%", product_for_search.search)
            )
            .fetch_all(db)
            .await
        },
        ProductSearchByType::FullName => {
            sqlx::query_as!(
                ProductStockLevelForDbResult,
                r#"SELECT
                    p.id,
                    p.sku,
                    p.brand,
                    p.name,
                    p.description,
                    p.price,
                    COALESCE(SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END), 0) as quantity
                FROM products p
                LEFT JOIN inventory_logs il
                ON p.id = il.product_id
                WHERE p.organization_id = $1
                AND p.display_name LIKE $2
                GROUP BY p.id, p.sku, p.brand, p.name, p.description, p.display_name, p.price
                ORDER BY p.display_name;"#,
                organization_id,
                format!("{}%", product_for_search.search)
            )
            .fetch_all(db)
            .await
        },
        ProductSearchByType::Brand => {
            sqlx::query_as!(
                ProductStockLevelForDbResult,
                r#"SELECT
                    p.id,
                    p.sku,
                    p.brand,
                    p.name,
                    p.description,
                    p.price,
                    COALESCE(SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END), 0) as quantity
                FROM products p
                LEFT JOIN inventory_logs il
                ON p.id = il.product_id
                WHERE p.organization_id = $1
                AND p.brand LIKE $2
                GROUP BY p.id, p.sku, p.brand, p.name, p.description, p.display_name, p.price
                ORDER BY p.display_name;"#,
                organization_id,
                format!("{}%", product_for_search.search)
            )
            .fetch_all(db)
            .await
        },
        ProductSearchByType::Name => {
            sqlx::query_as!(
                ProductStockLevelForDbResult,
                r#"SELECT
                    p.id,
                    p.sku,
                    p.brand,
                    p.name,
                    p.description,
                    p.price,
                    COALESCE(SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END), 0) as quantity
                FROM products p
                LEFT JOIN inventory_logs il
                ON p.id = il.product_id
                WHERE p.organization_id = $1
                AND p.display_name LIKE $2
                GROUP BY p.id, p.sku, p.brand, p.name, p.description, p.display_name, p.price
                ORDER BY p.display_name;"#,
                organization_id,
                format!("{}%", product_for_search.search)
            )
            .fetch_all(db)
            .await
        }
    }?;

    Ok(products.into_iter().map(|p| p.into()).collect())
}
// endregion: Methods
