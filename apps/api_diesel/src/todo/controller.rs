use actix_web::{get, post, web, HttpResponse, Responder};
use database_diesel_service::{service::todo::ListOptions, DatabaseService};
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

// #[get("/{id}")]
// async fn get_todo(
//     db_service: web::Data<DatabaseService>,
//     path: web::Path<String>,
// ) -> impl Responder {
//     let id = path.into_inner();
//     let data = super::service::get(db_service, &id).await.unwrap();

//     HttpResponse::Ok().json(TodoResponse {
//         id: data.id.to_string(),
//         title: data.title.to_string(),
//         checked: data.checked.into(),
//         create_time: data.create_time.to_rfc3339(),
//         modify_time: data.modify_time.to_rfc3339(),
//     })
// }

#[get("/list")]
async fn list(
    db_service: web::Data<DatabaseService>,
    web::Query(opts): web::Query<ListOptions>,
) -> impl Responder {
    let opts = ListOptions::set_from_obj(opts);
    let result = db_service.todo_service.list(opts).await.unwrap();
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
    let data = db_service.todo_service.create(&data.title).await.unwrap();
    web::Json(data)
}
