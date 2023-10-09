use super::ModelManager;
use crate::model::{common::RowWithId, permissions::Permissions, Result};
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
pub async fn provision_organization(
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

    let user = sqlx::query_as!(
        RowWithId,
        "INSERT INTO users (display_name, username, password, organization_id) \
          VALUES ($1, $2, $3, $4)\
          RETURNING id;",
        "default",
        "default",
        "default",
        organization.id
    )
    .fetch_one(db)
    .await?;

    sqlx::query!(
        "INSERT INTO user_permissions (user_id, permission_id) \
        VALUES ($1, $2);",
        user.id,
        Permissions::OrganizationAll as i32
    )
    .execute(db)
    .await?;

    tx.commit().await?;

    Ok(())
}
// endregion: Methods
