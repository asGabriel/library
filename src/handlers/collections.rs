use crate::domain::{collections::{Collection, CreateCollection}, errors::Result};

use super::Handler;

impl Handler {
    pub async fn create_collection(&self, collection: CreateCollection) -> Result<Collection> {
        self.collection_repository.create_collection(collection).await
    }
}