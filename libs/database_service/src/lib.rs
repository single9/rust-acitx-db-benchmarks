pub mod schema;
pub mod service;

use sqlx::PgPool;

use self::service::todo::TodoService;

trait BuildDatabaseService {
    fn new(pool: &PgPool) -> Self;
}

#[derive(Debug, Clone)]
pub struct DatabaseService {
    pub pool: Box<PgPool>,
    pub todo_service: TodoService,
}

impl DatabaseService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: Box::new(pool.clone()),
            todo_service: TodoService::new(&pool),
        }
    }
}
