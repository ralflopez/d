use crate::ctx::Ctx;
use crate::model::ModelManager;

use super::error::Result;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_valid::Valid;
use serde::Deserialize;
use serde_json::{json, Value};
use validator::Validate;

pub fn routes_inventory_deposit(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/inventory/deposits", post(deposit_handler))
        .with_state(mm)
}

#[derive(Debug, Deserialize, Validate)]
struct InventoryDepositPayloadItem {
    #[validate(required)]
    quantity: Option<i64>,
    #[validate(required)]
    product_id: Option<i64>,
    #[validate(required)]
    price: Option<f64>,
    #[validate(required(message = "is required"))]
    warehouse_id: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
struct InventoryDepositPayload {
    #[validate]
    items: Vec<InventoryDepositPayloadItem>,
}

async fn deposit_handler(
    State(_mm): State<ModelManager>,
    Json(body): Json<InventoryDepositPayload>,
) -> Result<Json<Value>> {
    let ctx = Ctx::new(1, 1);

    println!("{:?}", body);
    let response = Json(json!({
        "result": {
            "healthy": true
        }
    }));

    Ok(response)
}
