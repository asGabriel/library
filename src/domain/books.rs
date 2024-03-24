use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub book_id: Uuid,
    pub author_id: Option<Uuid>,
    pub collection_id: Option<Uuid>,
    pub name: String,
    pub genre: Genre,
    pub lang: Lang,
    pub rating: f64,
    pub creation_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateBook {
    pub name: String,
    pub author_id: Uuid,
    pub collection_id: Option<Uuid>,
    pub genre: Genre,
    pub lang: Lang,
    pub rating: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBook {
    pub name: Option<String>,
    pub author_id: Option<Uuid>,
    pub collection_id: Option<Uuid>,
    pub genre: Option<Genre>,
    pub lang: Option<Lang>,
    pub rating: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "genre", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Genre {
    Romance,
    Mystery,
    ScienceFiction,
    Fantasy,
    NonFiction,
    History,
    Biography,
    Poetry,
    Thriller,
    SelfHelp,
}
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "lang", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Lang {
    English,
    Portuguese,
    Spanish,
    German,
}
