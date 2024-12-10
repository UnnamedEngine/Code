use unen_backend::database::{self, redis::RedisPool};

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

    // Database Migrations
    database::check_for_migrations()
        .await
        .expect("An error occurred while running migrations.");

    // Database Connector
    let database_pool = database::connect()
        .await
        .expect("Database connection failed.");

    // Redis Connection
    let redis_pool = RedisPool::new(None);

    let app_config = unen_backend::app_setup(
        database_pool,
        redis_pool,
    );

    log::info!("Starting Axum HTTP server!");

    // Init App
    let app = unen_backend::app_config(app_config);

    // Run App with hypr
    let bind_address = dotenvy::var("BIND_ADDR")
        .expect("`BIND_ADDR` not set.");
    let self_address = dotenvy::var("SELF_ADDR")
        .expect("`SELF_ADDR` not set.");
    log::info!("Listening at: {}", self_address);
    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .expect("Failed to bind App listener.");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve App.");
}
