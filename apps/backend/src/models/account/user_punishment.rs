use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct UserPunishment {
    pub id: i32,
    pub user_id: i32,
    pub punishment_id: i32,
    pub applied_at: NaiveDateTime,
    pub duration: i64,
}
