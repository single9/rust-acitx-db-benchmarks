use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DatabaseConnection {
    app_name: String,
    database_host: String,
    database_port: u32,
    database_username: String,
    database_password: String,
    database_dbname: String,
    database_pool_max: u32,
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
                .parse::<u32>()
                .unwrap(),
            database_username: env::var("DATABASE_USERNAME").unwrap_or("postgres".to_string()),
            database_password: env::var("DATABASE_PASSWORD").unwrap_or("postgres".to_string()),
            database_dbname: env::var("DATABASE_DBNAME").unwrap_or("todo".to_string()),
            database_pool_max: env::var("DATABASE_POOL_MAX")
                .unwrap_or("10".to_string())
                .parse::<u32>()
                .unwrap(),
            app_name: "rust-sqlx".to_string(),
        }
    }

    /// Set database connection's application_name
    /// ```
    /// let db = DatabaseConnection::new()
    ///     .app_name("my-rust")
    ///     .connect();
    /// ```
    pub fn app_name(&mut self, app_name: String) -> &Self {
        self.app_name = app_name;
        self
    }

    /// Set database connection's application_name
    /// ```
    /// let db = DatabaseConnection::new()
    ///     .connect();
    /// ```
    pub fn connect(&self) -> PgPool {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}?application_name=rust-diesel",
            self.database_username,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_dbname
        );
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        // Create connection pool
        Pool::builder()
            .min_idle(Some(1))
            .max_size(self.database_pool_max)
            .build(manager)
            .expect("Failed to create pool.")
    }
}
