use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

pub mod models;
pub mod db;
mod tests;

/// Start the logger using ENV values.
pub fn start_logger() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
}

#[tokio::main]
async fn main() {
    // Read env values
    dotenvy::dotenv().ok();
    // Start logger
    start_logger();

    // Read `DATABASE_URL` from .env
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("failed to connect to the database");

    match pool {
        Ok(pool) => {
            if let Err(e) = sqlx::migrate!()
                .run(&pool)
                .await
                .context("failed to run migrations") {
                    log::error!("{}", e);
                }
        },
        Err(e) => {
            log::error!("{}", e);
        },
    }
}
