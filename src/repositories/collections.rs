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
    async fn get_collection_by_id(&self, collection_id: Uuid) -> Result<Option<Collection>>;
    async fn list_collections(&self) -> Result<Vec<Collection>>;
    async fn delete_collection_by_id(&self, collection_id: Uuid) -> Result<Option<Collection>>;
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

    async fn get_collection_by_id(&self, collection_id: Uuid) -> Result<Option<Collection>> {
        let collection = sqlx::query_as!(
            Collection,
            r#"
            SELECT * FROM COLLECTIONS WHERE DELETION_TIME IS NULL AND COLLECTION_ID = $1
            "#,
            collection_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(collection)
    }

    async fn list_collections(&self) -> Result<Vec<Collection>> {
        let collections = sqlx::query_as!(
            Collection,
            r#"
            SELECT * FROM COLLECTIONS WHERE DELETION_TIME IS NULL
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(collections)
    }

    async fn delete_collection_by_id(&self, collection_id: Uuid) -> Result<Option<Collection>> {
        let collection = sqlx::query_as!(
            Collection,
            r#"
            UPDATE COLLECTIONS SET
                UPDATED_AT = NOW(),
                DELETION_TIME = NOW()
            WHERE
                COLLECTION_ID = $1
                AND DELETION_TIME IS NULL
            RETURNING
                *
            "#,
            collection_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(collection)
    }
}
