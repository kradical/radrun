use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

use crate::user::UserRow;

pub struct InsertSession {
    pub user_id: i64,
}

#[allow(dead_code)]
pub struct SessionRow {
    pub id: Uuid,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct PsqlSessionStore {
    db: Pool<Postgres>,
}

impl PsqlSessionStore {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

impl PsqlSessionStore {
    pub async fn create(&self, params: InsertSession) -> Result<SessionRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            SessionRow,
            "
            INSERT INTO session
            (user_id)
            VALUES
            ($1)
            RETURNING *
        ",
            params.user_id,
        )
        .fetch_one(&self.db)
        .await?)
    }

    pub async fn delete(&self, id: Uuid) -> Result<SessionRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            SessionRow,
            "
            DELETE FROM session
            WHERE id = ($1)
            RETURNING *
        ",
            id,
        )
        .fetch_one(&self.db)
        .await?)
    }

    pub async fn get_user_by_session_id(
        &self,
        session_id: Uuid,
    ) -> Result<UserRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            UserRow,
            "
            SELECT rr_user.* FROM rr_user
            JOIN session
            ON rr_user.id = session.user_id
            WHERE session.id = $1",
            session_id
        )
        .fetch_one(&self.db)
        .await?)
    }
}
