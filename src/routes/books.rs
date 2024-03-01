use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{extract::State, routing::post, Json, Router};
use uuid::Uuid;

use crate::domain::books::CreateBook;
use crate::domain::errors::Result;
use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/books",
        Router::new()
            .route("/", post(create_books))
            .route("/:book_id", get(get_book_by_id)),
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
