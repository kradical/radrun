use std::{
    error::Error,
    fmt::{self},
    sync::Arc,
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
    user::{PsqlUserStore, UserInsert, UserRow},
};

pub struct SignUpParams {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub struct SignUpResult {
    pub user: UserRow,
    pub session: SessionRow,
}

pub struct PsqlAuthRepo {
    pub session_store: Arc<PsqlSessionStore>,
    pub user_store: Arc<PsqlUserStore>,
}

#[derive(Debug)]
struct PasswordHashError(password_hash::Error);

impl Error for PasswordHashError {}

impl fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PsqlAuthRepo {
    pub async fn get_user_by_session_id(
        &self,
        session_id: Uuid,
    ) -> Result<UserRow, Box<dyn Error>> {
        self.session_store.get_user_by_session_id(session_id).await
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<SessionRow, Box<dyn Error>> {
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

    pub async fn sign_up(&self, params: SignUpParams) -> Result<SignUpResult, Box<dyn Error>> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(params.password.as_bytes(), &salt)
            .map_err(|e| PasswordHashError(e))?
            .to_string();

        let user = self
            .user_store
            .create(UserInsert {
                first_name: params.first_name,
                last_name: params.last_name,
                email: params.email,
                password_hash,
            })
            .await?;

        let session = self
            .session_store
            .create(InsertSession { user_id: user.id })
            .await?;

        Ok(SignUpResult { user, session })
    }

    pub async fn delete_session(&self, session_id: Uuid) -> Result<SessionRow, Box<dyn Error>> {
        self.session_store.delete(session_id).await
    }
}
