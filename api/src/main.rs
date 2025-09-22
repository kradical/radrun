use std::error::Error;

use axum::{Router, routing::get};

use crate::account_store::{AccountStore, PsqlAccountStore};

mod account_store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://@localhost/radrun";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let account_store = PsqlAccountStore::new(pool);

    // INSERT
    let new_acc = account_store::InsertAccount {
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
    let updates = account_store::UpdateAccount {
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

    let app = Router::new().route("/", get(|| async { "Health Check" }));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
