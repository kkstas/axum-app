use crate::Result;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/data", get(api_data))
}

async fn api_data() -> Result<Json<Value>> {
    println!("->> {:<12} - api_data", "HANDLER");

    let body = Json(json!({
        "result": {
           "success": true,
        },
        "from": "async fn api_data() in routes_data.rs"
    }));
    Ok(body)
}
