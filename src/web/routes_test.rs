use crate::model::ModelManager;
pub use crate::web::error::{Error, Result};
use axum::routing::get;
use axum::{extract::State, Json, Router};
use serde_json::{json, Value};

pub fn test_routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/test", get(test_handler))
        .with_state(mm)
}

async fn test_handler(State(_mm): State<ModelManager>) -> Result<Json<Value>> {
    let response = Json(json!({
        "result": {
            "healthy": true
        }
    }));

    Ok(response)
}
