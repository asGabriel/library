use axum::Router;

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/collections", Router::new())
}
