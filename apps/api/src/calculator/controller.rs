use crate::calculator;
use actix_web::{get, web, HttpResponse, Responder};
use database_service::DatabaseService;

#[get("/test2")]
async fn test2(db_service: web::Data<DatabaseService>) -> impl Responder {
    let result = calculator::service::test_db(db_service, 180).await.unwrap();

    HttpResponse::Ok().body(format!("Test2: {}", result))
}

#[get("/test3")]
async fn test3(db_service: web::Data<DatabaseService>) -> impl Responder {
    let result = calculator::service::test_db(db_service, 230).await.unwrap();

    HttpResponse::Ok().body(format!("Test3: {}", result))
}
