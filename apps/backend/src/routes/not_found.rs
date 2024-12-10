use axum::{http::StatusCode, response::IntoResponse};

#[allow(dead_code)]
pub async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found".to_string(),
    )
}
