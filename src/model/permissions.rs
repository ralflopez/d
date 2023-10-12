use super::ModelManager;
use crate::{
    ctx::Ctx,
    model::{Error, Result},
};

// region: Enums
#[derive(Debug, PartialEq, Eq)]
pub enum Permissions {
    None,
    SuperUser = 1,
    OrganizationAll = 2,
}

impl TryFrom<i64> for Permissions {
    type Error = ();

    fn try_from(v: i64) -> core::result::Result<Self, Self::Error> {
        match v {
            x if x == Permissions::SuperUser as i64 => Ok(Permissions::SuperUser),
            x if x == Permissions::OrganizationAll as i64 => Ok(Permissions::OrganizationAll),
            _ => Err(()),
        }
    }
}
// endregion: Enums

// region: Methods
pub async fn has_permission(
    ctx: &Ctx,
    mm: &ModelManager,
    permission: Permissions,
    user_id: i64,
) -> Result<()> {
    let permissions = get_permissions_by_user_id(ctx, mm, user_id).await?;

    let valid = permissions
        .iter()
        .any(|p| *p == permission || *p == Permissions::SuperUser);
    if !valid {
        return Err(Error::Unauhtorized("Invalid permission".to_string()));
    }

    Ok(())
}

async fn get_permissions_by_user_id(
    _ctx: &Ctx,
    mm: &ModelManager,
    id: i64,
) -> Result<Vec<Permissions>> {
    let db = mm.db();

    let permissions: Vec<Permissions> = sqlx::query!(
        r#"SELECT p.id FROM user_permissions up
           INNER JOIN users u ON u.id = up.user_id
           INNER JOIN permissions p ON p.id = up.permission_id
           WHERE up.user_id = $1
           GROUP BY up.user_id, p.id;"#,
        id
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|p| p.id)
    .map(|p| match p.try_into() {
        Err(_) => Permissions::None,
        Ok(p) => p,
    })
    .collect();

    Ok(permissions)
}
// endregion: Methods
