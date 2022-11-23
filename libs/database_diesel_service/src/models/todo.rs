use crate::schema::todo;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, PartialEq, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub checked: bool,
    pub create_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = todo)]
pub struct NewTodo {
    pub title: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = todo)]
pub struct UpdateTodo {
    pub title: String,
    pub checked: bool,
}
