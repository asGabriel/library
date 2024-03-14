use uuid::Uuid;

use crate::domain::{
    collections::{Collection, CreateCollection},
    errors::{Error, Result},
};

use super::Handler;

impl Handler {
    pub async fn create_collection(&self, collection: CreateCollection) -> Result<Collection> {
        self.collection_repository
            .create_collection(collection)
            .await
    }

    pub async fn get_collection_by_id(&self, collection_id: Uuid) -> Result<Collection> {
        self.collection_repository
            .get_collection_by_id(collection_id)
            .await?
            .ok_or(Error::CollectionNotFound(collection_id))
    }
}
