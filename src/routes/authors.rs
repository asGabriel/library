use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use uuid::Uuid;

use crate::{
    domain::authors::{CreateAuthor, UpdateAuthor},
    repositories::authors::AuthorRepository,
};

pub(super) fn configure_routes() -> Router<Arc<dyn AuthorRepository + Send + Sync>> {
    Router::new().nest(
        "/authors",
        Router::new()
            .route("/", post(create_author))
            .route("/:author_id", get(get_author_by_id))
            .route("/:author_id", patch(update_author_by_id))
            .route("/:author_id", delete(delete_author_by_id)),
    )
}

async fn create_author(
    State(repository): State<Arc<dyn AuthorRepository + Send + Sync>>,
    Json(author): Json<CreateAuthor>,
) -> impl IntoResponse {
    if let Ok(author) = repository.create_author(author).await {
        (StatusCode::CREATED, Json::from(Some(author)))
    } else {
        (StatusCode::UNPROCESSABLE_ENTITY, Json::from(None))
    }
}

async fn get_author_by_id(
    State(repository): State<Arc<dyn AuthorRepository + Send + Sync>>,
    Path(author_id): Path<Uuid>,
) -> impl IntoResponse {
    if let Ok(author) = repository.get_author_by_id(author_id).await {
        if let Some(author) = author {
            (StatusCode::OK, Json::from(Some(author)))
        } else {
            (StatusCode::NOT_FOUND, Json::from(None))
        }
    } else {
        (StatusCode::UNPROCESSABLE_ENTITY, Json::from(None))
    }
}

async fn update_author_by_id(
    State(repository): State<Arc<dyn AuthorRepository + Send + Sync>>,
    Path(author_id): Path<Uuid>,
    Json(author): Json<UpdateAuthor>,
) -> impl IntoResponse {
    if let Ok(author) = repository.update_author_by_id(author, author_id).await {
        if let Some(author) = author {
            (StatusCode::OK, Json::from(Some(author)))
        } else {
            (StatusCode::NOT_FOUND, Json::from(None))
        }
    } else {
        (StatusCode::UNPROCESSABLE_ENTITY, Json::from(None))
    }
}

async fn delete_author_by_id() {}
