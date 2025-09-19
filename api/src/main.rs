use sqlx::types::time::OffsetDateTime;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://@localhost/radrun";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    // INSERT
    let new_acc = InsertAccount {
        first_name: "bloop".to_string(),
        last_name: "bloop".to_string(),
        email: "bloop2@bloop.bloop".to_string(),
        password_hash: "thisisahash".to_string(),
    };
    let created = create(&pool, new_acc).await?;
    println!("created: {:?}", created);

    // READ
    let gotten = get(&pool, 1).await?;
    println!("read: {:?}", gotten);

    // UPDATE
    let updates = UpdateAccount {
        first_name: "BLOOP2".to_string(),
        last_name: "BLOOP2".to_string(),
    };
    let updated = update(&pool, 1, updates).await?;
    println!("updated: {:?}", updated);

    // LIST
    let listed = list(&pool).await?;
    println!("listed: {:?}", listed);

    Ok(())
}

#[derive(sqlx::FromRow, Debug)]
struct AccountRow {
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
    // TODO: do the hashing
    password_hash: String,

    // TODO: Use chrono... or this fine??
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

struct InsertAccount {
    first_name: String,
    last_name: String,
    email: String,
    password_hash: String,
}

struct UpdateAccount {
    first_name: String,
    last_name: String,
}

async fn create(pool: &sqlx::PgPool, params: InsertAccount) -> Result<AccountRow, Box<dyn Error>> {
    let row = sqlx::query_as!(
        AccountRow,
        "
            INSERT INTO account
            (first_name, last_name, email, password_hash, created_at, updated_at)
            VALUES
            ($1, $2, $3, $4, NOW(), NOW())
            RETURNING *
        ",
        params.first_name,
        params.last_name,
        params.email,
        params.password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

async fn get(pool: &sqlx::PgPool, id: i64) -> Result<AccountRow, Box<dyn Error>> {
    let row = sqlx::query_as!(AccountRow, "SELECT * FROM account WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

async fn list(pool: &sqlx::PgPool) -> Result<Vec<AccountRow>, Box<dyn Error>> {
    let rows = sqlx::query_as!(AccountRow, "SELECT * FROM account")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

async fn update(
    pool: &sqlx::PgPool,
    id: i64,
    updates: UpdateAccount,
) -> Result<AccountRow, Box<dyn Error>> {
    let row = sqlx::query_as!(
        AccountRow,
        "UPDATE account
        SET first_name = $2, last_name = $3
        WHERE id = $1
        RETURNING *",
        id,
        updates.first_name,
        updates.last_name
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}
