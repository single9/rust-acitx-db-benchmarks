pub extern crate sqlx;

use dotenv::dotenv;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::env;

pub struct DatabaseConnection {
    app_name: String,
    database_host: String,
    database_port: u16,
    database_username: String,
    database_password: String,
    database_dbname: String,
}

impl DatabaseConnection {
    /// Create a new database instance with the given Environment variables
    /// - DATABASE_HOST
    /// - DATABASE_PORT
    /// - DATABASE_USERNAME
    /// - DATABASE_PASSWORD
    /// - DATABASE_DBNAME
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            database_host: env::var("DATABASE_HOST").unwrap_or("127.0.0.1".to_string()),
            database_port: env::var("DATABASE_PORT")
                .unwrap_or("5432".to_string())
                .parse::<u16>()
                .unwrap(),
            database_username: env::var("DATABASE_USERNAME").unwrap_or("postgres".to_string()),
            database_password: env::var("DATABASE_PASSWORD").unwrap_or("postgres".to_string()),
            database_dbname: env::var("DATABASE_DBNAME").unwrap_or("todo".to_string()),
            app_name: "rust-sqlx".to_string(),
        }
    }

    /// Set database connection's application_name
    /// ```
    /// let db = DatabaseConnection::new()
    ///     .app_name("my-rust")
    ///     .connect()
    ///     .await?;
    /// ```
    pub fn app_name(&mut self, app_name: String) -> &Self {
        self.app_name = app_name;
        self
    }

    /// Set database connection's application_name
    /// ```
    /// let db = DatabaseConnection::new()
    ///     .connect()
    ///     .await?;
    /// ```
    pub async fn connect(&self) -> Result<PgPool, sqlx::Error> {
        let option = PgConnectOptions::new()
            .host(&self.database_host)
            .port(self.database_port)
            .username(&self.database_username)
            .password(&self.database_password)
            .database(&self.database_dbname)
            .application_name(&self.app_name);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(option)
            .await?;

        Ok(pool)
    }
}
