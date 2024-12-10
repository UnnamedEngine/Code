use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};

use crate::{database::redis::RedisPool, routes::ApiError};

pub fn routes() -> Router {
    Router::new()
        .route("/status", get(status_get))
}

async fn status_get(
    Extension(database): Extension<sqlx::Pool<sqlx::Postgres>>,
    Extension(redis): Extension<RedisPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let results = crate::database::models::user_item::Status::list(&database, &redis)
        .await
        .map_err(ApiError::Database)?
        .into_iter()
        .map(crate::models::users::Status::from)
        .collect::<Vec<crate::models::users::Status>>();

    Ok(Json(results))
}
