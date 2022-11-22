use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    FromRow,
};

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub checked: bool,
    pub create_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,
}
