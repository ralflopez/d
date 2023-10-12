use super::{
    user::{create_user, UserForCreate},
    ModelManager,
};
use crate::{
    ctx::Ctx,
    model::{permissions::Permissions, Result},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: Structs
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct OrganizationForProvision {
    pub name: String,
    pub display_name: String,
}

// endregion: Structs

// region: Methods
pub async fn get_all_organizations(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Organization>> {
    let db = mm.db();

    let organizations = sqlx::query_as!(Organization, "SELECT * from organizations;")
        .fetch_all(db)
        .await?;

    Ok(organizations)
}

pub async fn register_organization(
    ctx: &Ctx,
    mm: &ModelManager,
    organization_for_provision: OrganizationForProvision,
) -> Result<()> {
    let db = mm.db();

    let tx = db.begin().await?;
    let organization = sqlx::query_as!(
        Organization,
        "INSERT INTO organizations (name, display_name) \
          VALUES ($1, $2) \
          RETURNING *;",
        organization_for_provision.name,
        organization_for_provision.display_name
    )
    .fetch_one(db)
    .await?;

    sqlx::query!(
        "INSERT INTO warehouses (name, organization_id) VALUES ($1, $2);",
        "default",
        organization.id
    )
    .execute(db)
    .await?;

    let user_id = create_user(
        ctx,
        mm,
        UserForCreate {
            display_name: "admin".to_string(),
            organization_id: organization.id,
            username: "admin".to_string(),
            password: "admin".to_string(),
        },
    )
    .await?;

    sqlx::query!(
        "INSERT INTO user_permissions (user_id, permission_id) \
        VALUES ($1, $2);",
        user_id,
        Permissions::OrganizationAll as i32
    )
    .execute(db)
    .await?;

    tx.commit().await?;

    Ok(())
}
// endregion: Methods
