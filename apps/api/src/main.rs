mod calculator;
mod db_service;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::db_service::DatabaseService;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/db")]
async fn db(db_service: web::Data<DatabaseService>) -> impl Responder {
    let row: (i64,) = database::sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(db_service.pool.as_ref())
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("Hello {}", row.0))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    let pool = database::connect().await.unwrap();
    let db_service = DatabaseService::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_service.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(echo)
            .service(db)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
