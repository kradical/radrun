use std::error::Error;

use axum::{Router, routing::get};

use crate::account::{PsqlAccountStore, get_account_router};

mod account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://@localhost/radrun";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let account_store = PsqlAccountStore::new(pool);

    let app = Router::new()
        .route("/", get(|| async { "Health Check" }))
        .nest("/account", get_account_router(account_store));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
