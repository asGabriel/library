use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{domain::{collections::CreateCollection, errors::Result}, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().route("/collections", post(create_collection))
}

async fn create_collection(
    State(handler): State<Handler>,
    Json(collection): Json<CreateCollection>
) -> Result<impl IntoResponse> {
    let collection = handler.create_collection(collection).await?;

    Ok(Json::from(collection))
}
