use sqlx::{PgPool, Result};

use crate::models::account::Punishment;

pub async fn insert_punishment(
    pool: &PgPool,
    slug: &str,
    default_duration: i64,
) -> Result<Punishment> {
    let record = sqlx::query_as!(
        Punishment,
        r#"
        INSERT INTO account.punishments (slug, default_duration)
        VALUES ($1, $2)
        RETURNING id, slug, default_duration, created_at, updated_at
        "#,
        slug,
        default_duration,
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_punishment_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Punishment>> {
    let record = sqlx::query_as!(
        Punishment,
        r#"
        SELECT id, slug, default_duration, created_at, updated_at
        FROM account.punishments
        WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn get_all_punishments(
    pool: &PgPool,
) -> Result<Vec<Punishment>> {
    let records = sqlx::query_as!(
        Punishment,
        r#"
        SELECT id, slug, default_duration, created_at, updated_at
        FROM account.punishments
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn update_punishment(
    pool: &PgPool,
    id: i32,
    slug: &str,
    default_duration: i64,
) -> Result<Option<Punishment>> {
    let record = sqlx::query_as!(
        Punishment,
        r#"
        UPDATE account.punishments
        SET slug = $1, default_duration = $2
        WHERE id = $3
        RETURNING id, slug, default_duration, created_at, updated_at
        "#,
        slug,
        default_duration,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_punishment(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Punishment>> {
    let record = sqlx::query_as!(
        Punishment,
        r#"
        DELETE FROM account.punishments
        WHERE id = $1
        RETURNING id, slug, default_duration, created_at, updated_at
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
