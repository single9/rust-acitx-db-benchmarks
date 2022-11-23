use crate::db_service::{
    models::{self, NewTodo, Todo},
    BuildDatabaseService,
};
use database_diesel::PgPool;
use diesel::prelude::*;
use serde::Deserialize;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    page: Option<i64>,
    per_page: Option<i64>,
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
    pub async fn create(&self, todo_title: &str) -> anyhow::Result<models::Todo> {
        use crate::db_service::schema::todo;

        let new_todo = NewTodo {
            title: todo_title.to_string(),
        };

        let mut conn = self.pool.get().unwrap();
        let res = diesel::insert_into(todo::table)
            .values(&new_todo)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub async fn get(&self, id: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn list<T>(&self, opts: T) -> anyhow::Result<Vec<Todo>>
    where
        T: Into<Option<ListOptions>>,
    {
        use crate::db_service::schema::todo::dsl::*;

        let opts: ListOptions = opts.into().unwrap_or(ListOptions::default());
        let limit = opts.per_page.unwrap();
        let offset = opts.page.unwrap() * limit;
        let mut conn = self.pool.get().unwrap();
        let res = todo
            .select((id, title, checked, create_time, modify_time))
            .order(create_time)
            .limit(limit)
            .offset(offset)
            .load::<Todo>(&mut conn)?;

        Ok(res)
    }
}
