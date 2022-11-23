use crate::{schema::todo::Todo, BuildDatabaseService};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    page: Option<i32>,
    per_page: Option<i32>,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            page: Some(0),
            per_page: Some(10),
        }
    }
}

impl ListOptions {
    pub fn set_from_obj(obj: ListOptions) -> Self {
        Self {
            page: Some(obj.page.unwrap_or(0)),
            per_page: Some(obj.per_page.unwrap_or(10)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TodoService {
    pub pool: Box<PgPool>,
}

impl BuildDatabaseService for TodoService {
    fn new(pool: &PgPool) -> Self {
        Self {
            pool: Box::new(pool.clone()),
        }
    }
}

impl TodoService {
    pub async fn create(&self, title: &str) -> Result<Todo, sqlx::Error> {
        let result = sqlx::query_as!(
            Todo,
            r"
            INSERT INTO todo (title) VALUES ($1) RETURNING *
            ",
            title
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(result)
    }

    pub async fn get(&self, title: &str) -> Result<Todo, sqlx::Error> {
        let result = sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE title = $1 LIMIT 1")
            .bind(title)
            .fetch_one(self.pool.as_ref())
            .await?;

        Ok(result)
    }

    pub async fn list<T>(&self, opts: T) -> Result<Vec<Todo>, sqlx::Error>
    where
        T: Into<Option<ListOptions>>,
    {
        let opts: ListOptions = opts.into().unwrap_or(ListOptions::default());
        let offset = opts.page.unwrap() * opts.per_page.unwrap();
        let result = sqlx::query_as::<_, Todo>("SELECT * FROM todo LIMIT $1 OFFSET $2")
            .bind(opts.per_page)
            .bind(offset)
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(result)
    }
}
