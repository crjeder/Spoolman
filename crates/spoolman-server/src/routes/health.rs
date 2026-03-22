use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub fn router() -> Router<crate::store::JsonStore> {
    Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn info(
    axum::extract::State(store): axum::extract::State<crate::store::JsonStore>,
) -> Json<spoolman_types::responses::InfoResponse> {
    Json(spoolman_types::responses::InfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        data_file: store.data_file_path().to_string_lossy().to_string(),
    })
}
