use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
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
    )
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CreateAuthor {
    name: String,
    date_of_birth: NaiveDate,
}

async fn create_author(Json(author): Json<CreateAuthor>) -> StatusCode {
    println!("{author:?}");
    StatusCode::CREATED
}

async fn get_author(Path(author_id): Path<Uuid>) -> (StatusCode, String) {
    (StatusCode::OK, format!("author_id = {author_id}"))
}
