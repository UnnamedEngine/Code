use sqlx::{PgPool, Result};

use crate::models::account::PunishmentTranslation;

pub async fn insert_punishment_translation(
    pool: &PgPool,
    punishment_id: i32,
    i18n_id: i32,
    title: &str,
    description: &str,
) -> Result<PunishmentTranslation> {
    let record = sqlx::query_as!(
        PunishmentTranslation,
        r#"
        INSERT INTO account.punishment_translations (punishment_id, i18n_id, title, description)
        VALUES ($1, $2, $3, $4)
        RETURNING punishment_id, i18n_id, title, description, created_at, updated_at
        "#,
        punishment_id,
        i18n_id,
        title,
        description,
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_punishment_translation_by_pk(
    pool: &PgPool,
    punishment_id: i32,
    i18n_id: i32,
) -> Result<Option<PunishmentTranslation>> {
    let record = sqlx::query_as!(
        PunishmentTranslation,
        r#"
        SELECT punishment_id, i18n_id, title, description, created_at, updated_at
        FROM account.punishment_translations
        WHERE punishment_id = $1 AND i18n_id = $2
        "#,
        punishment_id,
        i18n_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn get_all_punishment_translations(
    pool: &PgPool,
) -> Result<Vec<PunishmentTranslation>> {
    let records = sqlx::query_as!(
        PunishmentTranslation,
        r#"
        SELECT punishment_id, i18n_id, title, description, created_at, updated_at
        FROM account.punishment_translations
        ORDER BY punishment_id, i18n_id
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn update_punishment_translation(
    pool: &PgPool,
    punishment_id: i32,
    i18n_id: i32,
    title: &str,
    description: &str,
) -> Result<Option<PunishmentTranslation>> {
    let record = sqlx::query_as!(
        PunishmentTranslation,
        r#"
        UPDATE account.punishment_translations
        SET title = $1, description = $2
        WHERE punishment_id = $3 AND i18n_id = $4
        RETURNING punishment_id, i18n_id, title, description, created_at, updated_at
        "#,
        title,
        description,
        punishment_id,
        i18n_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_punishment_translation(
    pool: &PgPool,
    punishment_id: i32,
    i18n_id: i32,
) -> Result<Option<PunishmentTranslation>> {
    let record = sqlx::query_as!(
        PunishmentTranslation,
        r#"
        DELETE FROM account.punishment_translations
        WHERE punishment_id = $1 AND i18n_id = $2
        RETURNING punishment_id, i18n_id, title, description, created_at, updated_at
        "#,
        punishment_id,
        i18n_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
