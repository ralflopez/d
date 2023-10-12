use super::user::get_user_ids;
use super::ModelManager;
use crate::ctx::Ctx;
use crate::model::error::Result;
use sqlx::FromRow;

// region: Structs
#[derive(Debug, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
}
// region: Structs

// region: Methods
pub async fn get_category_by_id(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Category> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let category = sqlx::query_as!(
        Category,
        r#"SELECT id, name FROM categories WHERE id = $1 AND organization_id = $2;"#,
        id,
        organization_id
    )
    .fetch_one(db)
    .await?;

    Ok(category)
}

pub async fn get_all_categories(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Category>> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let categories = sqlx::query_as!(
        Category,
        r#"SELECT id, name FROM categories WHERE organization_id = $1
            ORDER BY name;"#,
        organization_id
    )
    .fetch_all(db)
    .await?;

    Ok(categories)
}

pub async fn create_category(ctx: &Ctx, mm: &ModelManager, name: String) -> Result<()> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    sqlx::query!(
        "INSERT INTO categories (name, organization_id) VALUES ($1, $2);",
        name,
        organization_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete_category(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    sqlx::query!(
        "DELETE FROM categories WHERE id = $1 AND organization_id = $2;",
        id,
        organization_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub struct CategoryForUpdate {
    pub id: i64,
    pub name: String,
}
pub async fn update_category(
    ctx: &Ctx,
    mm: &ModelManager,
    category_for_update: CategoryForUpdate,
) -> Result<Category> {
    let db = mm.db();
    let (_, organization_id) = get_user_ids(ctx, mm).await?;

    let category = sqlx::query_as!(
        Category,
        "UPDATE categories SET name = $1 WHERE id = $2 AND organization_id = $3 RETURNING id, name;",
        category_for_update.name,
        category_for_update.id,
        organization_id
    )
    .fetch_one(db)
    .await?;

    Ok(category)
}
// endregion: Methods

// Cases
// product is null
