use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

pub(super) fn configure_routes() -> Router {
    Router::new().nest(
        "/authors", Router::new()
        .route("/", post(create_author))
        .route("/:author_id", get(get_author))
        .route("/:author_id", patch(update_author))
    )
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CreateAuthor {
    name: String,
    date_of_birth: NaiveDate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateAuthor {
    name: Option<String>,
    date_of_birth: Option<NaiveDate> 
}

async fn create_author(Json(author): Json<CreateAuthor>) -> StatusCode {
    println!("{author:?}");
    StatusCode::CREATED
}

async fn get_author(Path(author_id): Path<Uuid>) -> (StatusCode, String) {
    (StatusCode::OK, format!("author_id = {author_id}"))
}

async fn update_author(Path(author_id): Path<Uuid>, Json(author): Json<UpdateAuthor>) -> (StatusCode, String) {
    (StatusCode::OK, format!("author_id = {}, struct: {:?}", author_id, author))
}