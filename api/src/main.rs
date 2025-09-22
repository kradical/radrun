use std::{error::Error, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::account_store::{AccountRow, InsertAccount, PsqlAccountStore, UpdateAccount};

mod account_store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://@localhost/radrun";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let account_store = PsqlAccountStore::new(pool);

    // INSERT
    let new_acc = InsertAccount {
        first_name: "bloop".to_string(),
        last_name: "bloop".to_string(),
        email: "bloop2@bloop.bloop".to_string(),
        password_hash: "thisisahash".to_string(),
    };
    let created = account_store.create(new_acc).await?;
    println!("created: {:?}", created);

    // READ
    let gotten = account_store.get(created.id).await?;
    println!("read: {:?}", gotten);

    // UPDATE
    let updates = UpdateAccount {
        first_name: "BLOOP2".to_string(),
        last_name: "BLOOP2".to_string(),
    };
    let updated = account_store.update(created.id, updates).await?;
    println!("updated: {:?}", updated);

    // LIST
    let listed = account_store.list().await?;
    println!("listed: {:?}", listed);

    for account in listed {
        account_store.delete(account.id).await?;
    }

    let app = Router::new()
        .route("/", get(|| async { "Health Check" }))
        .route("/account", post(create_account))
        .with_state(AppState {
            account_store: Arc::new(account_store),
        });

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    account_store: Arc<PsqlAccountStore>,
}

#[derive(Deserialize)]
struct CreateAccountReq {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct CreateAccountRes {
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
}

impl CreateAccountRes {
    fn from(row: AccountRow) -> Self {
        CreateAccountRes {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            email: row.email,
        }
    }
}

async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccountReq>,
) -> Result<Json<CreateAccountRes>, StatusCode> {
    let insert_row = InsertAccount {
        email: payload.email,
        first_name: payload.first_name,
        last_name: payload.last_name,
        password_hash: payload.password,
    };

    state
        .account_store
        .create(insert_row)
        .await
        .map(CreateAccountRes::from)
        .map(axum::Json)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}
