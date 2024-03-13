use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::{extract::State, Json, Router};
use uuid::Uuid;

use crate::domain::books::{CreateBook, UpdateBook};
use crate::domain::errors::Result;
use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/books",
        Router::new()
            .route("/", post(create_books))
            .route("/:book_id", get(get_book_by_id))
            .route("/", get(list_all_books))
            .route("/:book_id", patch(update_book_by_id))
            .route("/:book_id", delete(delete_book_by_id)),
    )
}

async fn create_books(
    State(handler): State<Handler>,
    Json(book): Json<CreateBook>,
) -> Result<impl IntoResponse> {
    let book = handler.create_book(book).await?;

    Ok(Json::from(book))
}

async fn get_book_by_id(
    State(handler): State<Handler>,
    Path(book_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let book = handler.get_book_by_id(book_id).await?;

    Ok(Json::from(book))
}

async fn list_all_books(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let books = handler.list_all_books().await?;

    Ok(Json::from(books))
}

async fn update_book_by_id(
    State(handler): State<Handler>,
    Path(book_id): Path<Uuid>,
    Json(book): Json<UpdateBook>,
) -> Result<impl IntoResponse> {
    let book = handler.update_book_by_id(book_id, book).await?;

    Ok(Json::from(book))
}

async fn delete_book_by_id(
    State(handler): State<Handler>,
    Path(book_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let book = handler.delete_book_by_id(book_id).await?;

    Ok(Json::from(book))
}
