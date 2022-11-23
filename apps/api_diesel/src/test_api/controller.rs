use crate::test_api;

use actix_web::{get, web, HttpResponse, Responder};

#[get("/hello/{name}")]
async fn test_name(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let res = test_api::service::test_service(name);
    HttpResponse::Ok().body(res)
}
