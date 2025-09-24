use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::user::user_store::{PsqlUserStore, UserInsert, UserRow, UserUpdate};

#[derive(Clone)]
struct UserRouteState {
    store: Arc<PsqlUserStore>,
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "user.ts")]

struct UserCreateReq {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "user.ts")]
struct UserUpdateReq {
    first_name: String,
    last_name: String,
}

impl UserUpdateReq {
    fn to(&self) -> UserUpdate {
        UserUpdate {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
        }
    }
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "user.ts")]
struct UserRes {
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRes {
    fn from(row: &UserRow) -> Self {
        UserRes {
            id: row.id,
            first_name: row.first_name.clone(),
            last_name: row.last_name.clone(),
            email: row.email.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "user.ts")]
struct UsersRes {
    data: Vec<UserRes>,
}

impl UsersRes {
    fn from(rows: &Vec<UserRow>) -> Self {
        let data = rows.iter().map(UserRes::from).collect();

        UsersRes { data }
    }
}

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

async fn sign_up(
    State(state): State<UserRouteState>,
    Json(req): Json<UserCreateReq>,
) -> Result<Json<UserRes>, StatusCode> {
    // TODO: how does lib handle changes to default params? poorly? backwards compat??
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    state
        .store
        .create(UserInsert {
            first_name: req.first_name,
            last_name: req.last_name,
            email: req.email,
            password_hash,
        })
        .await
        .map(|row: UserRow| UserRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_user(
    State(state): State<UserRouteState>,
    Path(id): Path<i64>,
) -> Result<Json<UserRes>, StatusCode> {
    state
        .store
        .get_id(id)
        .await
        .map(|row: UserRow| UserRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_user(
    State(state): State<UserRouteState>,
    Path(id): Path<i64>,
) -> Result<Json<UserRes>, StatusCode> {
    state
        .store
        .delete(id)
        .await
        .map(|row: UserRow| UserRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_user(
    State(state): State<UserRouteState>,
    Path(id): Path<i64>,
    Json(req): Json<UserUpdateReq>,
) -> Result<Json<UserRes>, StatusCode> {
    state
        .store
        .update(id, req.to())
        .await
        .map(|row: UserRow| UserRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn list_users(State(state): State<UserRouteState>) -> Result<Json<UsersRes>, StatusCode> {
    state
        .store
        .list()
        .await
        .map(|rows: Vec<UserRow>| UsersRes::from(&rows))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn get_user_router(store: Arc<PsqlUserStore>) -> Router {
    Router::new()
        .route("/", post(sign_up))
        .route("/", get(list_users))
        .route("/{id}", get(get_user))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
        .with_state(UserRouteState { store })
}
