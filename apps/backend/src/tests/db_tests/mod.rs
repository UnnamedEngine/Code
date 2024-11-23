mod public_tests;
mod account_tests;

#[cfg(test)]
pub mod utils {
    use sqlx::{Executor, PgPool};

    /// Setup an isolated test database.
    pub async fn setup_isolated_db() -> (PgPool, String) {
        // Load the .env file
        dotenvy::dotenv().ok();

        // Base database URL
        let base_db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        // Get raw database URL
        let raw_db_url = base_db_url
            .rsplit_once('/')
            .map(|(base, _)| base)
            .expect("DATABASE_URL is not in the expected format");

        // Unique test database name
        let db_name = format!("test_db_{}", uuid::Uuid::new_v4());

        // Connect to main database and create an isolated database
        let admin_pool = PgPool::connect(&base_db_url).await.unwrap();
        let query = format!(r#"CREATE DATABASE "{}""#, db_name);
        admin_pool
            .execute(query.as_str())
            .await
            .unwrap();

        // Newly created database URL
        let test_db_url = format!("{}/{}", raw_db_url, db_name);

        // Connect to newly created test database
        let test_pool = PgPool::connect(&test_db_url).await.unwrap();

        // Run migrations for newly created test database
        sqlx::migrate!()
            .run(&test_pool)
            .await
            .expect("failed to run migrations for newly created test database");

        (test_pool, db_name)
    }

    // Cleanup an isolated test database by name.
    pub async fn cleanup_isolated_db(db_name: &str) {
        // Base database URL
        let base_db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        // Connect to main database
        let admin_pool = PgPool::connect(&base_db_url).await.unwrap();

        // Force disconnect active connections
        let terminate_connections_query = format!(
            r#"
            SELECT pg_terminate_backend(pg_stat_activity.pid)
            FROM pg_stat_activity
            WHERE pg_stat_activity.datname = '{}' AND pid <> pg_backend_pid();
            "#,
            db_name
        );
        admin_pool
            .execute(terminate_connections_query.as_str())
            .await
            .expect("Failed to terminate active connections");

        // Delete test daatbase
        let query = format!(r#"DROP DATABASE "{}""#, db_name);
        admin_pool
            .execute(query.as_str())
            .await
            .unwrap();
    }
}
