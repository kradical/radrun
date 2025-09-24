use std::{error::Error, sync::Arc};

use axum::{Router, routing::get};

use crate::auth::get_auth_router;
use crate::user::{PsqlUserStore, get_user_router};

mod auth;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://@localhost/radrun";

    // TODO: share a pool
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    let pool2 = sqlx::postgres::PgPool::connect(url).await?;

    let user_store = Arc::new(PsqlUserStore::new(pool));

    let api = Router::new()
        .route("/", get(|| async { "Health Check" }))
        .nest("/auth", get_auth_router(user_store.clone(), pool2))
        .nest("/user", get_user_router(user_store.clone()));

    let app = Router::new().nest("/api", api);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
