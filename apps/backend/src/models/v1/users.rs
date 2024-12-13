use super::ids::Base62Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(from = "Base62Id")]
#[serde(into = "Base62Id")]
pub struct UserId(pub u64);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub id: i32,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

use crate::database::models::user_item::Status as DBStatus;
impl From<DBStatus> for Status {
    fn from(data: DBStatus) -> Self {
        Self {
            id: data.id,
            slug: data.slug,
            created_at: data.created_at,
        }
    }
}

impl Status {
    fn from_full(db_status: DBStatus) -> Self {
        Self {
            id: db_status.id,
            slug: db_status.slug,
            created_at: db_status.created_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub status: Option<Status>,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

use crate::database::models::user_item::User as DBUser;
impl From<DBUser> for User {
    fn from(data: DBUser) -> Self {
        Self {
            id: data.id.into(),
            status: None,
            email: data.email,
            username: data.username,
            display_name: data.display_name,
            avatar_url: None,
            created_at: data.created_at,
        }
    }
}

impl User {
    pub fn from_full(db_user: DBUser, db_status: DBStatus) -> Self {
        Self {
            id: UserId::from(db_user.id),
            status: Some(Status::from_full(db_status)),
            email: db_user.email,
            username: db_user.username,
            display_name: db_user.display_name,
            avatar_url: None,
            created_at: db_user.created_at,
        }
    }
}
