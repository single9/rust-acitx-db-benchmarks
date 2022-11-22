use crate::db_service::{service::todo::ListOptions, DatabaseService};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TodoResponse {
    id: String,
    title: String,
    checked: bool,
    create_time: String,
    modify_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoBody {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoBody2 {
    title: Option<String>,
}

#[get("/{id}")]
async fn get_todo(
    db_service: web::Data<DatabaseService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let data = super::service::get(db_service, &id).await.unwrap();

    HttpResponse::Ok().json(TodoResponse {
        id: data.id.to_string(),
        title: data.title.to_string(),
        checked: data.checked.into(),
        create_time: data.create_time.to_rfc3339(),
        modify_time: data.modify_time.to_rfc3339(),
    })
}

#[get("/list")]
async fn list(
    db_service: web::Data<DatabaseService>,
    web::Query(opts): web::Query<ListOptions>,
) -> impl Responder {
    let opts = Some(opts);
    let result = super::service::list(db_service, opts).await.unwrap();
    let list: Vec<TodoResponse> = result
        .iter()
        .map(|data| TodoResponse {
            id: data.id.to_string(),
            title: data.title.to_string(),
            checked: data.checked.into(),
            create_time: data.create_time.to_rfc3339(),
            modify_time: data.modify_time.to_rfc3339(),
        })
        .collect();

    HttpResponse::Ok().json(list)
}

#[post("/")]
async fn create(
    db_service: web::Data<DatabaseService>,
    data: web::Json<TodoBody>,
) -> impl Responder {
    let data = super::service::create(db_service, &data.title)
        .await
        .unwrap();

    let result = TodoResponse {
        id: data.id.to_string(),
        title: data.title.to_string(),
        checked: data.checked.into(),
        create_time: data.create_time.to_rfc3339(),
        modify_time: data.modify_time.to_rfc3339(),
    };

    HttpResponse::Ok().json(result)
}
