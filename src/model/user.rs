use crate::crypt;
use crate::ctx::Ctx;
use crate::model::common::RowWithId;
use crate::model::{Error, Result};

use super::ModelManager;

// region: Structs
pub struct UserForCreate {
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub organization_id: i64,
}

// endregion: Structs

// region: Methods
pub async fn create_user(
    _ctx: &Ctx,
    mm: &ModelManager,
    user_for_create: UserForCreate,
) -> Result<i64> {
    let db = mm.db();

    let user = sqlx::query_as!(
        RowWithId,
        "INSERT INTO users (display_name, username, password, organization_id) \
          VALUES ($1, $2, $3, $4)\
          RETURNING id;",
        user_for_create.display_name,
        user_for_create.username,
        user_for_create.password,
        user_for_create.organization_id
    )
    .fetch_one(db)
    .await?;

    Ok(user.id)
}

pub async fn verify_password(
    _ctx: Ctx,
    mm: &ModelManager,
    id: i64,
    password: String,
) -> Result<()> {
    let db = mm.db();

    let password_hash = sqlx::query!(r#"SELECT password FROM users WHERE id = $1;"#, id)
        .fetch_one(db)
        .await?
        .password;

    match crypt::compare_hash(&password_hash, &password) {
        Err(_) => Err(Error::Unauhtorized("Password does not match".to_string())),
        Ok(_) => Ok(()),
    }
}

pub async fn verify_organization(
    _ctx: &Ctx,
    mm: &ModelManager,
    user_id: i64,
    org_id: i64,
) -> Result<()> {
    let db = mm.db();

    let query_result = sqlx::query!("SELECT organization_id FROM users WHERE id = $1;", user_id)
        .fetch_one(db)
        .await?;

    match query_result.organization_id {
        None => Err(Error::Unauhtorized("Organization id not found".to_string())),
        Some(result_id) => {
            if result_id != org_id {
                return Err(Error::Unauhtorized("Invalid organization id".to_string()));
            }

            Ok(())
        }
    }
}

pub async fn get_user_ids(ctx: &Ctx, mm: &ModelManager) -> Result<(i64, i64)> {
    let db = mm.db();

    let user_id = ctx
        .user_id()
        .ok_or_else(|| Error::Unauhtorized("User id not found".to_string()))?;

    let query_result = sqlx::query!("SELECT organization_id FROM users WHERE id = $1;", user_id)
        .fetch_one(db)
        .await?;

    match query_result.organization_id {
        None => Err(Error::Unauhtorized("Organization id not found".to_string())),
        Some(org_id) => Ok((user_id, org_id)),
    }
}

// endregion: Methods
