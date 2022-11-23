use actix_web::web;
use database_service::DatabaseService;

pub async fn test_db(db_service: web::Data<DatabaseService>, num: i64) -> anyhow::Result<i64> {
    let row: (i64,) = database::sqlx::query_as("SELECT $1")
        .bind(num)
        .fetch_one(db_service.pool.as_ref())
        .await
        .unwrap();

    Ok(row.0)
}
