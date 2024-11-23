use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug,FromRow)]
pub struct Punishment {
    pub id: i32,
    pub slug: String,
    pub default_duration: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
