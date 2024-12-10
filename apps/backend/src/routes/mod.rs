use axum::{http::StatusCode, routing::get, Router};
use thiserror::Error;

mod v1;

mod index;
mod not_found;

pub fn root() -> Router {
    Router::new()
        .route("/", get(index::index))
        .nest("/v1", v1::routes())
        .fallback(not_found::not_found)
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database Error: {0}")]
    Database(#[from] crate::database::models::DatabaseError),
}

impl From<ApiError> for StatusCode {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Database(..) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
