use std::time::Duration;

use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, Connection, PgConnection, Postgres};

pub async fn connect() -> Result<sqlx::PgPool, sqlx::Error> {
    log::info!("Initializing database connection");
    let database_url = dotenvy::var("DATABASE_URL")
        .expect("`DATABASE_URL` not set");
    let pool = PgPoolOptions::new()
        .min_connections(
            dotenvy::var("DATABASE_MIN_CONNECTIONS")
                    .ok()
                    .and_then(|x| x.parse().ok())
                    .unwrap_or(0),
        )
        .max_connections(
            dotenvy::var("DATABASE_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|x| x.parse().ok())
                    .unwrap_or(16),
        )
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn check_for_migrations() -> Result<(), sqlx::Error> {
    let database_url = dotenvy::var("DATABASE_URL")
        .expect("`DATABASE_URL` not set");
    let database_url = database_url.as_str();
    if !Postgres::database_exists(database_url).await? {
        log::info!("Creating database...");
        Postgres::create_database(database_url).await?;
    }

    log::info!("Applying migrations...");

    let mut conn = PgConnection::connect(database_url).await?;
    sqlx::migrate!()
        .run(&mut conn)
        .await
        .expect("Error while running database migrations!");

    Ok(())
}
