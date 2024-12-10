use axum::response::IntoResponse;
use serde_json::json;

pub async fn index() -> impl IntoResponse {
    json!({
        "name": "unen-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "documentation": "https://docs.unnamedengine.com",
        "about": "Welcome!"
    });
}
