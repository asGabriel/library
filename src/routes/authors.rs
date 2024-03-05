use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use uuid::Uuid;

use crate::{
    domain::{
        authors::{CreateAuthor, UpdateAuthor},
        errors::Result,
    },
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/authors",
        Router::new()
            .route("/", post(create_author))
            .route("/:author_id", get(get_author_by_id))
            .route("/:author_id", patch(update_author_by_id))
            .route("/:author_id", delete(delete_author_by_id))
            .route("/", get(list_authors)),
    )
}

async fn create_author(
    State(handler): State<Handler>,
    Json(author): Json<CreateAuthor>,
) -> Result<impl IntoResponse> {
    let author = handler.create_author(author).await?;

    Ok(Json::from(author))
}

async fn get_author_by_id(
    State(handler): State<Handler>,
    Path(author_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let author = handler.get_author_by_id(author_id).await?;

    Ok(Json(author))
}

async fn update_author_by_id(
    State(handler): State<Handler>,
    Path(author_id): Path<Uuid>,
    Json(author): Json<UpdateAuthor>,
) -> Result<impl IntoResponse> {
    let author = handler.update_author_by_id(author_id, author).await?;

    Ok(Json(author))
}

async fn delete_author_by_id(
    State(handler): State<Handler>,
    Path(author_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let author = handler.delete_author_by_id(author_id).await?;

    Ok(Json(author))
}

async fn list_authors(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let authors = handler.list_authors().await?;

    Ok(Json::from(authors))
}
