use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use std::error::Error;

#[derive(sqlx::FromRow, Clone)]
pub struct UserRow {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserInsert {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}

pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
}

pub struct PsqlUserStore {
    db: Pool<Postgres>,
}

impl PsqlUserStore {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

impl PsqlUserStore {
    pub async fn create(&self, params: UserInsert) -> Result<UserRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            UserRow,
            "
            INSERT INTO rr_user
            (first_name, last_name, email, password_hash, created_at, updated_at)
            VALUES
            ($1, $2, $3, $4, NOW(), NOW())
            RETURNING *
        ",
            params.first_name,
            params.last_name,
            params.email,
            params.password_hash
        )
        .fetch_one(&self.db)
        .await?)
    }

    pub async fn update(&self, id: i64, params: UserUpdate) -> Result<UserRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            UserRow,
            "UPDATE rr_user
            SET first_name = $2, last_name = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING *",
            id,
            params.first_name,
            params.last_name
        )
        .fetch_one(&self.db)
        .await?)
    }

    pub async fn delete(&self, id: i64) -> Result<UserRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            UserRow,
            "DELETE FROM rr_user
            WHERE id = $1
            RETURNING *",
            id
        )
        .fetch_one(&self.db)
        .await?)
    }

    pub async fn get_id(&self, id: i64) -> Result<UserRow, Box<dyn Error>> {
        Ok(
            sqlx::query_as!(UserRow, "SELECT * FROM rr_user WHERE id = $1", id)
                .fetch_one(&self.db)
                .await?,
        )
    }

    pub async fn get_email(&self, email: &str) -> Result<UserRow, Box<dyn Error>> {
        Ok(
            sqlx::query_as!(UserRow, "SELECT * FROM rr_user WHERE email = $1", email)
                .fetch_one(&self.db)
                .await?,
        )
    }

    pub async fn list(&self) -> Result<Vec<UserRow>, Box<dyn Error>> {
        Ok(sqlx::query_as!(UserRow, "SELECT * FROM rr_user")
            .fetch_all(&self.db)
            .await?)
    }
}
