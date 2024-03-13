use crate::domain::{
    collections::{Collection, CreateCollection},
    errors::Result,
};
use mockall::automock;
use uuid::Uuid;

use super::SqlxRepository;

#[automock]
#[async_trait::async_trait]
pub trait CollectionRepository {
    async fn create_collection(&self, collection: CreateCollection) -> Result<Collection>;
}

#[async_trait::async_trait]
impl CollectionRepository for SqlxRepository {
    async fn create_collection(&self, collection: CreateCollection) -> Result<Collection> {
        let collection = sqlx::query_as!(
            Collection,
            r#"
            INSERT INTO COLLECTIONS
                (
                    COLLECTION_ID,
                    NAME,
                    BOOK_IDS
                )
            VALUES
                (
                    $1, $2, $3

                )
            RETURNING
                *
            "#,
            Uuid::new_v4(),
            collection.name,
            collection.book_ids.as_slice()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(collection)
    }
}
