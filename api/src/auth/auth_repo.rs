use std::{
    error::Error,
    fmt::{self, Display},
};

use argon2::{
    Argon2,
    password_hash::{
        self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};
use uuid::Uuid;

use crate::{
    auth::session_store::{InsertSession, PsqlSessionStore, SessionRow},
    user::{PsqlUserStore, UserRow},
};

pub struct AuthRepo {
    session_store: PsqlSessionStore,
    user_store: PsqlUserStore,
}

impl AuthRepo {
    pub fn new(session_store: PsqlSessionStore, user_store: PsqlUserStore) -> Self {
        Self {
            session_store,
            user_store,
        }
    }
}

#[derive(Debug)]
struct PasswordHashError(password_hash::Error);

impl Error for PasswordHashError {}

impl fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AuthRepo {
    pub async fn get_user_by_session_id(
        &self,
        session_id: Uuid,
    ) -> Result<UserRow, Box<dyn Error>> {
        self.session_store.get_user_by_session_id(session_id).await
    }

    pub async fn login(&self, email: &str, password: String) -> Result<SessionRow, Box<dyn Error>> {
        let user_row = self.user_store.get_email(email).await?;

        PasswordHash::new(&user_row.password_hash)
            .and_then(|hash| Argon2::default().verify_password(password.as_bytes(), &hash))
            .map_err(|e| PasswordHashError(e))?;

        return self
            .session_store
            .create(InsertSession {
                user_id: user_row.id,
            })
            .await;
    }
}
