use uuid::Uuid;

use crate::domain::{
    books::{Book, CreateBook, UpdateBook},
    errors::{Error, Result},
};

use super::Handler;

impl Handler {
    pub async fn create_book(&self, book: CreateBook) -> Result<Book> {
        let created_book = self.book_repository.create_book(&book).await?;

        if let Some(collection) = book.collection_id {
            let collection_validated = self.get_collection_by_id(collection).await?;

            let _ = self.collection_repository.insert_book_into_collection(
                collection_validated.collection_id,
                created_book.book_id,
            ).await;
        }

        Ok(created_book)
    }

    pub async fn get_book_by_id(&self, book_id: Uuid) -> Result<Book> {
        self.book_repository
            .get_book_by_id(book_id)
            .await?
            .ok_or(Error::BookNotFound(book_id))
    }

    pub async fn list_all_books(&self) -> Result<Vec<Book>> {
        self.book_repository.list_all_books().await
    }

    pub async fn update_book_by_id(&self, book_id: Uuid, book: UpdateBook) -> Result<Book> {
        self.book_repository
            .update_book_by_id(book_id, book)
            .await?
            .ok_or(Error::BookNotFound(book_id))
    }

    pub async fn delete_book_by_id(&self, book_id: Uuid) -> Result<Book> {
        self.book_repository
            .delete_book_by_id(book_id)
            .await?
            .ok_or(Error::BookNotFound(book_id))
    }
}
