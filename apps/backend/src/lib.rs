use axum::{Extension, Router};
use database::redis::RedisPool;

pub mod database;
pub mod models;
pub mod routes;

#[derive(Clone)]
pub struct AppConfig {
    pub database_pool: sqlx::Pool<sqlx::Postgres>,
    pub redis_pool: RedisPool,
}

pub fn app_setup(
    database_pool: sqlx::Pool<sqlx::Postgres>,
    redis_pool: RedisPool,
) -> AppConfig {
    log::info!(
        "Starting UNEN Backend on {}",
        dotenvy::var("BIND_ADDR").unwrap(),
    );

    AppConfig {
        database_pool,
        redis_pool,
    }
}

pub fn app_config(app_config: AppConfig) -> Router {
    Router::new()
        .merge(routes::root())
        .layer(Extension(app_config.database_pool))
        .layer(Extension(app_config.redis_pool))
}
