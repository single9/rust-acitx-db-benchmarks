use actix_web::{get, post, web, HttpResponse, Responder};
use database_service::{service::todo::ListOptions, DatabaseService};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TodoBody {
    title: String,
}

#[get("/{title}")]
async fn get_todo(
    db_service: web::Data<DatabaseService>,
    path: web::Path<String>,
) -> impl Responder {
    let title = path.into_inner();
    let data = super::service::get(db_service, &title).await.unwrap();

    HttpResponse::Ok().json(data)
}

#[get("/list")]
async fn list(
    db_service: web::Data<DatabaseService>,
    web::Query(opts): web::Query<ListOptions>,
) -> impl Responder {
    let opts = ListOptions::set_from_obj(opts);
    let result = super::service::list(db_service, Some(opts)).await.unwrap();

    HttpResponse::Ok().json(result)
}

#[post("/")]
async fn create(
    db_service: web::Data<DatabaseService>,
    data: web::Json<TodoBody>,
) -> impl Responder {
    let data = db_service.todo_service.create(&data.title).await.unwrap();

    HttpResponse::Ok().json(data)
}
