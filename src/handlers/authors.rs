use uuid::Uuid;

use crate::domain::{
    authors::{Author, CreateAuthor, UpdateAuthor},
    errors::{Error, Result},
};

use super::Handler;

impl Handler {
    pub async fn create_author(&self, author: CreateAuthor) -> Result<Author> {
        self.author_repository.create_author(author).await
    }

    pub async fn get_author_by_id(&self, author_id: Uuid) -> Result<Author> {
        self.author_repository
            .get_author_by_id(author_id)
            .await?
            .ok_or(Error::AuthorNotFound(author_id))
    }

    pub async fn update_author_by_id(
        &self,
        author_id: Uuid,
        author: UpdateAuthor,
    ) -> Result<Author> {
        self.author_repository
            .update_author_by_id(author, author_id)
            .await?
            .ok_or(Error::AuthorNotFound(author_id))
    }

    pub async fn delete_author_by_id(&self, author_id: Uuid) -> Result<Author> {
        self.author_repository
            .delete_author_by_id(author_id)
            .await?
            .ok_or(Error::AuthorNotFound(author_id))
    }

    pub async fn list_authors(&self) -> Result<Vec<Author>> {
        self.author_repository.list_authors().await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::repositories::{
        authors::MockAuthorRepository, books::MockBookRepository,
        collections::MockCollectionRepository,
    };

    #[tokio::test]
    async fn test_get_author() {
        let mut author_repository = MockAuthorRepository::new();
        let book_repository = MockBookRepository::new();
        let collection_repository = MockCollectionRepository::new();
        let author_id = Uuid::default();

        author_repository.expect_get_author_by_id().returning(|_| {
            Err(Error::DatabaseError(sqlx::Error::Protocol(
                Default::default(),
            )))
        });

        let handler = Handler::new(
            Arc::new(author_repository),
            Arc::new(book_repository),
            Arc::new(collection_repository),
        );

        let error = handler.get_author_by_id(author_id).await.unwrap_err();

        assert!(matches!(error, Error::DatabaseError(..)));
    }
}
