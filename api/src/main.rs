use std::{error::Error, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::account::{PsqlAccountStore, get_account_router};

mod account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://@localhost/radrun";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let account_store = Arc::new(PsqlAccountStore::new(pool));

    let api = Router::new()
        .route("/", get(|| async { "Health Check" }))
        .nest("/auth", get_auth_router(account_store.clone()))
        .nest("/account", get_account_router(account_store.clone()));

    let app = Router::new().nest("/api", api);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct AuthRouteState {
    store: Arc<PsqlAccountStore>,
}

#[derive(Deserialize)]
struct LoginReq {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginRes {
    session_id: Uuid,
}

pub fn get_auth_router(store: Arc<PsqlAccountStore>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(AuthRouteState { store })
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

    // TODO: how to better unnest results.. combine error handling??
    PasswordHash::new(&account_row.password_hash)
        .map(|hash| argon2.verify_password(req.password.as_bytes(), &hash))
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .map(|_| LoginRes {
            session_id: Uuid::new_v4(),
        })
        .map(Json)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}
