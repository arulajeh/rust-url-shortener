use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Url {
    pub id: i64,
    pub shorten: String,
    pub url: String,
    pub created_at: Option<NaiveDateTime>,
    pub counter: Option<i64>,
}
