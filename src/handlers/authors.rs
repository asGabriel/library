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
}
