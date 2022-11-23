use actix_web::web;

use crate::db_service::DatabaseService;

pub async fn test_db(db_service: web::Data<DatabaseService>, num: i64) -> anyhow::Result<i64> {
    Ok(123)
}
