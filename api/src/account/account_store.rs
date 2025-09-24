use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use std::error::Error;

#[derive(sqlx::FromRow)]
pub struct AccountRow {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct InsertAccount {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}

pub struct UpdateAccount {
    pub first_name: String,
    pub last_name: String,
}

pub struct PsqlAccountStore {
    db: Pool<Postgres>,
}

impl PsqlAccountStore {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

impl PsqlAccountStore {
    pub async fn create(&self, params: InsertAccount) -> Result<AccountRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            AccountRow,
            "
            INSERT INTO account
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

    pub async fn update(
        &self,
        id: i64,
        params: UpdateAccount,
    ) -> Result<AccountRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            AccountRow,
            "UPDATE account
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

    pub async fn delete(&self, id: i64) -> Result<AccountRow, Box<dyn Error>> {
        Ok(sqlx::query_as!(
            AccountRow,
            "DELETE FROM account
            WHERE id = $1
            RETURNING *",
            id
        )
        .fetch_one(&self.db)
        .await?)
    }

    pub async fn get_id(&self, id: i64) -> Result<AccountRow, Box<dyn Error>> {
        Ok(
            sqlx::query_as!(AccountRow, "SELECT * FROM account WHERE id = $1", id)
                .fetch_one(&self.db)
                .await?,
        )
    }

    pub async fn get_email(&self, email: &str) -> Result<AccountRow, Box<dyn Error>> {
        Ok(
            sqlx::query_as!(AccountRow, "SELECT * FROM account WHERE email = $1", email)
                .fetch_one(&self.db)
                .await?,
        )
    }

    pub async fn list(&self) -> Result<Vec<AccountRow>, Box<dyn Error>> {
        Ok(sqlx::query_as!(AccountRow, "SELECT * FROM account")
            .fetch_all(&self.db)
            .await?)
    }
}
