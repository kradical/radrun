# radrun

## Setup

1. Install postgres
   https://www.postgresql.org/
   `brew install postgresql@17`

2. Install rust
   https://www.rust-lang.org/tools/install
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

3. Install sqlx-cli
   https://crates.io/crates/sqlx-cli
   `cargo install sqlx-cli --no-default-features --features native-tls,postgres`

3. Create the database
   https://crates.io/crates/sqlx-cli
   `sqlx database create` or `createdb radrun`

4. Run the migrations
   https://crates.io/crates/sqlx-cli
   `sqlx migrate run`

5. Run the api server
   `cargo run`
