use actix_web::web;

use crate::db_service::{schema::todo::Todo, service::todo::ListOptions, DatabaseService};

pub async fn list(
    db_service: web::Data<DatabaseService>,
    opts: Option<ListOptions>,
) -> anyhow::Result<Vec<Todo>> {
    let result = db_service.todo_service.list(opts).await?;
    Ok(result)
}

pub async fn create(db_service: web::Data<DatabaseService>, title: &str) -> anyhow::Result<Todo> {
    let result = db_service.todo_service.create(title).await?;
    Ok(result)
}

pub async fn get(db_service: web::Data<DatabaseService>, id: &str) -> anyhow::Result<Todo> {
    let result = db_service.todo_service.get(id).await?;
    Ok(result)
}
