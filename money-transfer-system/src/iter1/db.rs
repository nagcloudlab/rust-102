// db/mod.rs
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
