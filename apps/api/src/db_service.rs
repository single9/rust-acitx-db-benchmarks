use database::sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct DatabaseService {
    pub pool: Box<PgPool>,
}

impl DatabaseService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Box::new(pool),
        }
    }
}
