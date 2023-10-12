#[derive(Clone)]
pub struct Ctx {
    user_id: Option<i64>,
    organization_id: Option<i64>,
}

impl Ctx {
    pub fn new(user_id: i64, organization_id: i64) -> Self {
        Self {
            user_id: Some(user_id),
            organization_id: Some(organization_id),
        }
    }

    pub fn empty() -> Self {
        Self {
            user_id: None,
            organization_id: None,
        }
    }

    pub fn user_id(&self) -> Option<i64> {
        self.user_id
    }

    pub fn organization_id(&self) -> Option<i64> {
        self.organization_id
    }
}
