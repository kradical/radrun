use std::{error::Error, sync::Arc};

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use ts_rs::TS;
use uuid::Uuid;

use crate::account::PsqlAccountStore;

#[derive(Clone)]
struct AuthRouteState {
    db: Pool<Postgres>,
    store: Arc<PsqlAccountStore>,
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "auth.ts")]
struct LoginReq {
    email: String,
    password: String,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "auth.ts")]
struct LoginRes {
    session_id: Uuid,
}

pub fn get_auth_router(store: Arc<PsqlAccountStore>, db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(AuthRouteState { db, store })
}

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};

async fn login(
    State(state): State<AuthRouteState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<LoginRes>, StatusCode> {
    let account_row = state
        .store
        .get_email(&req.email)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // TODO: how does lib handle changes to default params? poorly? backwards compat??
    let argon2 = Argon2::default();
    let verify_result = PasswordHash::new(&account_row.password_hash)
        .and_then(|hash| argon2.verify_password(req.password.as_bytes(), &hash));

    if verify_result.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    create_session(
        &state.db,
        InsertSession {
            account_id: account_row.id,
        },
    )
    .await
    .map(|session_row| LoginRes {
        session_id: session_row.id,
    })
    .map(Json)
    .inspect_err(|e| println!("{:?}", e))
    .map_err(|_| StatusCode::UNAUTHORIZED)
}

struct InsertSession {
    account_id: i64,
}

struct SessionRow {
    id: Uuid,
    account_id: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

async fn create_session(
    db: &Pool<Postgres>,
    params: InsertSession,
) -> Result<SessionRow, Box<dyn Error>> {
    Ok(sqlx::query_as!(
        SessionRow,
        "
            INSERT INTO session
            (account_id)
            VALUES
            ($1)
            RETURNING *
        ",
        params.account_id,
    )
    .fetch_one(db)
    .await?)
}
