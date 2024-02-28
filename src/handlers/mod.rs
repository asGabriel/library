use std::sync::Arc;
pub mod authors;
pub mod books;
use crate::repositories::{authors::AuthorRepository, books::BookRepository};

#[derive(Clone)]
pub struct Handler {
    author_repository: Arc<dyn AuthorRepository + Send + Sync>,
    book_repository: Arc<dyn BookRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        author_repository: Arc<dyn AuthorRepository + Send + Sync>,
        book_repository: Arc<dyn BookRepository + Send + Sync>,
    ) -> Self {
        Self {
            author_repository,
            book_repository,
        }
    }
}
