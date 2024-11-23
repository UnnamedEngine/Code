use sqlx::{PgPool, Result};

use crate::models::account::StatusTranslation;

pub async fn insert_status_translation(
    pool: &PgPool,
    status_id: i32,
    i18n_id: i32,
    title: &str,
    description: &str
) -> Result<StatusTranslation> {
    let record = sqlx::query_as!(
        StatusTranslation,
        r#"
        INSERT INTO account.status_translations (status_id, i18n_id, title, description)
        VALUES ($1, $2, $3, $4)
        RETURNING status_id, i18n_id, title, description, created_at, updated_at
        "#,
        status_id,
        i18n_id,
        title,
        description,
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_status_translation_by_pk(
    pool: &PgPool,
    status_id: i32,
    i18n_id: i32,
) -> Result<Option<StatusTranslation>> {
    let record = sqlx::query_as!(
        StatusTranslation,
        r#"
        SELECT status_id, i18n_id, title, description, created_at, updated_at
        FROM account.status_translations
        WHERE status_id = $1 AND i18n_id = $2
        "#,
        status_id,
        i18n_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn get_all_status_translations(
    pool: &PgPool,
) -> Result<Vec<StatusTranslation>> {
    let records = sqlx::query_as!(
        StatusTranslation,
        r#"
        SELECT status_id, i18n_id, title, description, created_at, updated_at
        FROM account.status_translations
        ORDER BY status_id, i18n_id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn update_status_translation(
    pool: &PgPool,
    status_id: i32,
    i18n_id: i32,
    title: &str,
    description: &str,
) -> Result<Option<StatusTranslation>> {
    let record = sqlx::query_as!(
        StatusTranslation,
        r#"
        UPDATE account.status_translations
        SET title = $1, description = $2
        WHERE status_id = $3 AND i18n_id = $4
        RETURNING status_id, i18n_id, title, description, created_at, updated_at
        "#,
        title,
        description,
        status_id,
        i18n_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_status_translation(
    pool: &PgPool,
    status_id: i32,
    i18n_id: i32,
) -> Result<Option<StatusTranslation>> {
    let record = sqlx::query_as!(
        StatusTranslation,
        r#"
        DELETE FROM account.status_translations
        WHERE status_id = $1 AND i18n_id = $2
        RETURNING status_id, i18n_id, title, description, created_at, updated_at
        "#,
        status_id,
        i18n_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
