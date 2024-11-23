use sqlx::{PgPool, Result};

use crate::models::account::Status;

pub async fn insert_status(
    pool: &PgPool,
    slug: &str,
) -> Result<Status> {
    let record = sqlx::query_as!(
        Status,
        r#"
        INSERT INTO account.status (slug)
        VALUES ($1)
        RETURNING id, slug, created_at, updated_at
        "#,
        slug,
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_status_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Status>> {
    let record = sqlx::query_as!(
        Status,
        r#"
        SELECT id, slug, created_at, updated_at
        FROM account.status
        WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn get_all_status(
    pool: &PgPool,
) -> Result<Vec<Status>> {
    let records = sqlx::query_as!(
        Status,
        r#"
        SELECT id, slug, created_at, updated_at
        FROM account.status
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn update_status(
    pool: &PgPool,
    id: i32,
    slug: &str,
) -> Result<Option<Status>> {
    let record = sqlx::query_as!(
        Status,
        r#"
        UPDATE account.status
        SET slug = $1
        WHERE id = $2
        RETURNING id, slug, created_at, updated_at
        "#,
        slug,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_status(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Status>> {
    let record = sqlx::query_as!(
        Status,
        r#"
        DELETE FROM account.status
        WHERE id = $1
        RETURNING id, slug, created_at, updated_at
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
