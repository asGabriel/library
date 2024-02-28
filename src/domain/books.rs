use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub book_id: String,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateBook {
    pub name: String,
    pub author_id: Uuid,
    pub collection_id: Uuid,
    pub genre: Genre,
    pub lang: Lang,
    pub rating: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "genre")]
#[warn(non_camel_case_types)]
pub enum Genre {
    ROMANCE,
    MYSTERY,
    SCIENCE_FICTION,
    FANTASY,
    NON_FICTION,
    HISTORY,
    BIOGRAPHY,
    POETRY,
    THRILLER,
    SELF_HELP,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "lang")]
pub enum Lang {
    ENGLISH,
    PORTUGUESE,
    SPANISH,
    GERMAN,
}
