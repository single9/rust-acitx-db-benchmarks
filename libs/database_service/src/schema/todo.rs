use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub checked: bool,
    pub create_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,
}
