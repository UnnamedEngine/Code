use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};

use crate::models::account::User;

pub async fn insert_user(
    pool: &PgPool,
    status_id: i32,
    email: &str,
    username: &str,
    display_name: &str,
) -> Result<User> {
    let record = sqlx::query_as!(
        User,
        r#"
        INSERT INTO account.users (status_id, email, username, display_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, status_id, email, username, display_name, created_at, updated_at, last_login
        "#,
        status_id,
        email,
        username,
        display_name,
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_user_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<User>> {
    let record = sqlx::query_as!(
        User,
        r#"
        SELECT id, status_id, email, username, display_name, created_at, updated_at, last_login
        FROM account.users
        WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn get_all_users(
    pool: &PgPool,
) -> Result<Vec<User>> {
    let records = sqlx::query_as!(
        User,
        r#"
        SELECT id, status_id, email, username, display_name, created_at, updated_at, last_login
        FROM account.users
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn update_user(
    pool: &PgPool,
    id: i32,
    status_id: i32,
    email: &str,
    username: &str,
    display_name: &str,
    last_login: Option<NaiveDateTime>,
) -> Result<Option<User>> {
    let record = sqlx::query_as!(
        User,
        r#"
        UPDATE account.users
        SET status_id = $1, email = $2, username = $3, display_name = $4, last_login = $5
        WHERE id = $6
        RETURNING id, status_id, email, username, display_name, created_at, updated_at, last_login
        "#,
        status_id,
        email,
        username,
        display_name,
        last_login,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_user(
    pool: &PgPool,
    id: i32,
) -> Result<Option<User>> {
    let record = sqlx::query_as!(
        User,
        r#"
        DELETE FROM account.users
        WHERE id = $1
        RETURNING id, status_id, email, username, display_name, created_at, updated_at, last_login
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
