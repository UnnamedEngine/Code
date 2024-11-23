use sqlx::{PgPool, Result};

use crate::models::public::I18n;

pub async fn insert_i18n(
    pool: &PgPool,
    written_name: &str,
    language_code: &str,
) -> Result<I18n> {
    let record = sqlx::query_as!(
        I18n,
        r#"
        INSERT INTO public.i18n (written_name, language_code)
        VALUES ($1, $2)
        RETURNING id, written_name, language_code
        "#,
        written_name,
        language_code
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

pub async fn get_i18n_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<I18n>> {
    let record = sqlx::query_as!(
        I18n,
        r#"
        SELECT id, written_name, language_code
        FROM public.i18n
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn get_all_i18n(
    pool: &PgPool,
) -> Result<Vec<I18n>> {
    let records = sqlx::query_as!(
        I18n,
        r#"
        SELECT id, written_name, language_code
        FROM public.i18n
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn update_i18n(
    pool: &PgPool,
    id: i32,
    written_name: &str,
    language_code: &str,
) -> Result<Option<I18n>> {
    let record = sqlx::query_as!(
        I18n,
        r#"
        UPDATE public.i18n
        SET written_name = $1, language_code = $2
        WHERE id = $3
        RETURNING id, written_name, language_code
        "#,
        written_name,
        language_code,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

pub async fn delete_i18n(
    pool: &PgPool,
    id: i32,
) -> Result<Option<I18n>> {
    let record = sqlx::query_as!(
        I18n,
        r#"
        DELETE FROM public.i18n
        WHERE id = $1
        RETURNING id, written_name, language_code
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}
