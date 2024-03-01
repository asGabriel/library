use uuid::Uuid;

use crate::domain::{
    books::{Book, CreateBook},
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
}
