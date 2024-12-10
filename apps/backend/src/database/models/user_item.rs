use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::redis::RedisPool;

use super::{DatabaseError, UserId};

use futures::TryStreamExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub id: i32,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,

    pub status_id: i32,

    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

impl Status {
    pub async fn list<'a, E>(
        exec: E,
        redis: &RedisPool,
    ) -> Result<Vec<Status>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let mut redis = redis.connect().await?;

        let res = redis
            .get_deserialized_from_json("account", "status")
            .await?;

        if let Some(res) = res {
            return Ok(res);
        }

        let result = sqlx::query!(
            "
            SELECT
                id,
                slug,
                created_at
            FROM account.status
            ORDER BY created_at
            ",
        )
        .fetch(exec)
        .map_ok(|s| Status {
            id: s.id,
            slug: s.slug,
            created_at: s.created_at,
        })
        .try_collect::<Vec<Status>>()
        .await?;

        redis
            .set_serialized_to_json(
                "account",
                "status",
                &result,
                None,
            )
            .await?;

        Ok(result)
    }
}

impl User {
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO account.users (
                id,
                status_id,
                email,
                username,
                display_name,
                created_at,
                last_login
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
            ",
            self.id as UserId,
            self.status_id,
            self.email,
            self.username,
            self.display_name,
            self.created_at,
            self.last_login,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}
