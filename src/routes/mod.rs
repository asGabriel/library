use axum::Router;
mod authors;
mod books;

pub(super) fn configure_routes() -> Router {
    authors::configure_routes()
}
