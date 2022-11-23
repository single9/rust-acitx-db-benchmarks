use crate::{
    models::todo::{NewTodo, Todo},
    BuildDatabaseService,
};
use database_diesel::PgPool;
use diesel::prelude::*;
use serde::Deserialize;

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
    pub async fn create(&self, todo_title: &str) -> Result<Todo, diesel::result::Error> {
        use crate::schema::todo;

        let new_todo = NewTodo {
            title: todo_title.to_string(),
        };

        let mut conn = self.pool.get().unwrap();

        diesel::insert_into(todo::table)
            .values(&new_todo)
            .get_result(&mut conn)
    }

    pub async fn get(&self, todo_title: &str) -> Result<Todo, diesel::result::Error> {
        use crate::schema::todo::dsl::*;

        let mut conn = self.pool.get().unwrap();

        todo.filter(title.eq(todo_title.to_string()))
            .limit(1)
            .first::<Todo>(&mut conn)
    }

    pub async fn list<T>(&self, opts: T) -> Result<Vec<Todo>, diesel::result::Error>
    where
        T: Into<Option<ListOptions>>,
    {
        use crate::schema::todo::dsl::*;

        let opts: ListOptions = opts.into().unwrap_or(ListOptions::default());
        let limit = opts.per_page.unwrap();
        let offset = opts.page.unwrap() * limit;
        let mut conn = self.pool.get().unwrap();

        todo.select((id, title, checked, create_time, modify_time))
            .limit(limit)
            .offset(offset)
            .load::<Todo>(&mut conn)
    }
}
