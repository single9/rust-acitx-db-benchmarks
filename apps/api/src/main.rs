mod calculator;
mod test_api;
mod todo;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use database::DatabaseConnection;
use database_service::DatabaseService;
use logger::log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    logger::init();

    let port = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .unwrap();
    let pool = DatabaseConnection::new().connect().await?;
    let db_service = DatabaseService::new(&pool);

    info!("Server started at http://127.0.0.1:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_service.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::scope("/api")
                    .service(test_api::init())
                    .service(calculator::init())
                    .service(todo::init()),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
