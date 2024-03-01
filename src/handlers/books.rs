use crate::domain::{
    books::{Book, CreateBook},
    errors::Result,
};

use super::Handler;

impl Handler {
    pub async fn create_book(&self, book: CreateBook) -> Result<Book> {
        self.book_repository.create_book(book).await
    }
}
