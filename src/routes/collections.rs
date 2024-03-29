use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domain::{collections::CreateCollection, errors::Result},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/collections",
        Router::new()
            .route("/collections", post(create_collection))
            .route("/:collection_id", get(get_collection_by_id))
            .route("/", get(list_collections))
            .route("/:collection_id", delete(delete_collection_by_id)),
    )
}

async fn create_collection(
    State(handler): State<Handler>,
    Json(collection): Json<CreateCollection>,
) -> Result<impl IntoResponse> {
    let collection = handler.create_collection(collection).await?;

    Ok(Json::from(collection))
}

async fn get_collection_by_id(
    State(handler): State<Handler>,
    Path(collection_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let collection = handler.get_collection_by_id(collection_id).await?;

    Ok(Json::from(collection))
}

async fn list_collections(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let collections = handler.list_collections().await?;

    Ok(Json::from(collections))
}

async fn delete_collection_by_id(
    State(handler): State<Handler>,
    Path(collection_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let collection = handler.delete_collection_by_id(collection_id).await?;

    Ok(Json::from(collection))
}
