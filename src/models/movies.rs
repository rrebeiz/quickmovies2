use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, Type};
#[derive(Debug, Serialize, FromRow, Decode)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub year: i32,
    pub runtime: i32,
    pub genres: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

#[derive(Debug, Deserialize)]
pub struct MovieCreateRequest {
    pub title: String,
    pub year: i32,
    pub runtime: i32,
    pub genres: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MovieUpdateRequest {
    pub title: Option<String>,
    pub year: Option<i32>,
    pub runtime: Option<i32>,
    pub genres: Option<Vec<String>>,
}
