use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::account::account_store::{AccountRow, InsertAccount, PsqlAccountStore, UpdateAccount};

#[derive(Clone)]
struct AccountRouteState {
    store: Arc<PsqlAccountStore>,
}

#[derive(Deserialize)]
struct CreateAccountReq {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl CreateAccountReq {
    fn to(&self) -> InsertAccount {
        InsertAccount {
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            password_hash: self.password.clone(),
        }
    }
}

#[derive(Deserialize)]
struct UpdateAccountReq {
    first_name: String,
    last_name: String,
}

impl UpdateAccountReq {
    fn to(&self) -> UpdateAccount {
        UpdateAccount {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
        }
    }
}

#[derive(Serialize)]
struct AccountRes {
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl AccountRes {
    fn from(row: &AccountRow) -> Self {
        AccountRes {
            id: row.id,
            first_name: row.first_name.clone(),
            last_name: row.last_name.clone(),
            email: row.email.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Serialize)]
struct AccountsRes {
    data: Vec<AccountRes>,
}

impl AccountsRes {
    fn from(rows: &Vec<AccountRow>) -> Self {
        let data = rows.iter().map(AccountRes::from).collect();

        AccountsRes { data }
    }
}

async fn create_account(
    State(state): State<AccountRouteState>,
    Json(req): Json<CreateAccountReq>,
) -> Result<Json<AccountRes>, StatusCode> {
    state
        .store
        .create(req.to())
        .await
        .map(|row: AccountRow| AccountRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_account(
    State(state): State<AccountRouteState>,
    Path(id): Path<i64>,
) -> Result<Json<AccountRes>, StatusCode> {
    state
        .store
        .get(id)
        .await
        .map(|row: AccountRow| AccountRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_account(
    State(state): State<AccountRouteState>,
    Path(id): Path<i64>,
) -> Result<Json<AccountRes>, StatusCode> {
    state
        .store
        .delete(id)
        .await
        .map(|row: AccountRow| AccountRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_account(
    State(state): State<AccountRouteState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateAccountReq>,
) -> Result<Json<AccountRes>, StatusCode> {
    state
        .store
        .update(id, req.to())
        .await
        .map(|row: AccountRow| AccountRes::from(&row))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn list_accounts(
    State(state): State<AccountRouteState>,
) -> Result<Json<AccountsRes>, StatusCode> {
    state
        .store
        .list()
        .await
        .map(|rows: Vec<AccountRow>| AccountsRes::from(&rows))
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn get_account_router(store: PsqlAccountStore) -> Router {
    Router::new()
        .route("/", post(create_account))
        .route("/", get(list_accounts))
        .route("/{id}", get(get_account))
        .route("/{id}", put(update_account))
        .route("/{id}", delete(delete_account))
        .with_state(AccountRouteState {
            store: Arc::new(store),
        })
}
