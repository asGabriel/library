use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domain::errors::Error, handlers::Handler};
mod authors;
mod books;
mod collections;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new()
        .merge(authors::configure_routes())
        .merge(books::configure_routes())
        .merge(collections::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::AuthorNotFound(author_id) => (
                StatusCode::NOT_FOUND,
                format!("Author {author_id} not found"),
            ),
            Self::BookNotFound(book_id) => {
                (StatusCode::NOT_FOUND, format!("Book {book_id} not found"))
            }
            Self::CollectionNotFound(collection_id) => (
                StatusCode::NOT_FOUND,
                format!("Collection {collection_id} not found"),
            ),
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
        }
        .into_response()
    }
}
