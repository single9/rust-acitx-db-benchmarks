use actix_web::{get, post, web, HttpResponse, Responder};
use database_diesel_service::{models::todo::NewTodo, service::todo::ListOptions, DatabaseService};

#[get("/{id}")]
async fn get_todo(
    db_service: web::Data<DatabaseService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let result = db_service.todo_service.get(&id).await.unwrap();

    HttpResponse::Ok().json(result)
}

#[get("/list")]
async fn list(
    db_service: web::Data<DatabaseService>,
    web::Query(opts): web::Query<ListOptions>,
) -> impl Responder {
    let opts = ListOptions::set_from_obj(opts);
    let result = db_service.todo_service.list(opts).await.unwrap();

    HttpResponse::Ok().json(result)
}

#[post("/")]
async fn create(
    db_service: web::Data<DatabaseService>,
    data: web::Json<NewTodo>,
) -> impl Responder {
    let result = db_service.todo_service.create(&data.title).await.unwrap();

    HttpResponse::Ok().json(result)
}
