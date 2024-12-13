use thiserror::Error;

pub mod ids;
pub mod user_item;

pub use ids::*;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Error while trying to generate random ID")]
    RandomId,
    #[error("Error while interacting with the cache: {0}")]
    Cache(#[from] redis::RedisError),
    #[error("Redis Pool Error: {0}")]
    RedisPool(#[from] deadpool_redis::PoolError),
    #[error("Error while serializing with the cache: {0}")]
    SerdeCacheError(#[from] serde_json::Error),
    #[error("Schema error: {0}")]
    SchemaError(String),
    #[error("Timeout when waiting for cache subscriber")]
    CacheTimeout,
}
