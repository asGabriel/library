use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    collection_id: String,
    name: String,
    book_ids: Vec<Uuid>,
    pub creation_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollection {
    pub name: String,
    pub book_ids: Vec<Uuid>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollection {
    pub name: Option<String>,
    pub book_ids: Option<Vec<Uuid>>
}
