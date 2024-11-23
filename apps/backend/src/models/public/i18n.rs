use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct I18n {
    pub id: i32,
    pub written_name: String,
    pub language_code: String,
}
