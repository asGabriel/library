use uuid::Uuid;

use crate::domain::{
    books::{Book, CreateBook, UpdateBook},
    errors::{Error, Result},
};

use super::Handler;

impl Handler {
    pub async fn create_book(&self, book: CreateBook) -> Result<Book> {
        self.book_repository.create_book(book).await
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
