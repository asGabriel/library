use axum::response::IntoResponse;
use axum::{extract::State, routing::post, Json, Router};

use crate::domain::books::CreateBook;
use crate::domain::errors::Result;
use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().route("/books", post(create_books))
}

async fn create_books(
    State(handler): State<Handler>,
    Json(book): Json<CreateBook>,
) -> Result<impl IntoResponse> {
    let book = handler.create_book(book).await?;

    Ok(Json::from(book))
}
