use sqlx::{PgPool, Result};

use crate::models::account::UserPunishment;

pub async fn insert_user_punishment(
    pool: &PgPool,
    user_id: i32,
    punishment_id: i32,
    duration: i64,
) -> Result<UserPunishment> {
    let record = sqlx::query_as!(
        UserPunishment,
        r#"
        INSERT INTO account.user_punishments (user_id, punishment_id, duration)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, punishment_id, applied_at, duration
        "#,
        user_id,
        punishment_id,
        duration,
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_user_punishment_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<UserPunishment>> {
    let record = sqlx::query_as!(
        UserPunishment,
        r#"
        SELECT id, user_id, punishment_id, applied_at, duration
        FROM account.user_punishments
        WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn get_all_user_punishments(
    pool: &PgPool,
) -> Result<Vec<UserPunishment>> {
    let records = sqlx::query_as!(
        UserPunishment,
        r#"
        SELECT id, user_id, punishment_id, applied_at, duration
        FROM account.user_punishments
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn update_user_punishment(
    pool: &PgPool,
    id: i32,
    user_id: i32,
    punishment_id: i32,
    duration: i64,
) -> Result<Option<UserPunishment>> {
    let record = sqlx::query_as!(
        UserPunishment,
        r#"
        UPDATE account.user_punishments
        SET user_id = $1, punishment_id = $2, duration = $3
        WHERE id = $4
        RETURNING id, user_id, punishment_id, applied_at, duration
        "#,
        user_id,
        punishment_id,
        duration,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_user_punishment(
    pool: &PgPool,
    id: i32,
) -> Result<Option<UserPunishment>> {
    let record = sqlx::query_as!(
        UserPunishment,
        r#"
        DELETE FROM account.user_punishments
        WHERE id = $1
        RETURNING id, user_id, punishment_id, applied_at, duration
        "#,
        id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
