use std::{error::Error, sync::Arc};

use axum::{Router, routing::get};

use crate::account::{PsqlAccountStore, get_account_router};
use crate::auth::get_auth_router;

mod account;
mod auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://@localhost/radrun";

    // TODO: share a pool
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    let pool2 = sqlx::postgres::PgPool::connect(url).await?;

    let account_store = Arc::new(PsqlAccountStore::new(pool));

    let api = Router::new()
        .route("/", get(|| async { "Health Check" }))
        .nest("/auth", get_auth_router(account_store.clone(), pool2))
        .nest("/account", get_account_router(account_store.clone()));

    let app = Router::new().nest("/api", api);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
