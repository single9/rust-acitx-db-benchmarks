extern crate dotenv;
pub extern crate sqlx;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let database_host = env::var("DATABASE_HOST").unwrap_or("127.0.0.1".to_string());
    let database_port = env::var("DATABASE_PORT").unwrap_or("5432".to_string());
    let database_username = env::var("DATABASE_USERNAME").unwrap_or("postgres".to_string());
    let database_password = env::var("DATABASE_PASSWORD").unwrap_or("postgres".to_string());
    let database_dbname = env::var("DATABASE_DBNAME").unwrap_or("postgres".to_string());
    let url = format!(
        "postgres://{database_username}:{database_password}@{database_host}:{database_port}/{database_dbname}"
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    Ok(pool)
}
