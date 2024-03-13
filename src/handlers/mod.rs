use std::sync::Arc;
pub mod authors;
pub mod books;
pub mod collections;
use crate::repositories::{
    authors::AuthorRepository, books::BookRepository, collections::CollectionRepository,
};

#[derive(Clone)]
pub struct Handler {
    author_repository: Arc<dyn AuthorRepository + Send + Sync>,
    book_repository: Arc<dyn BookRepository + Send + Sync>,
    collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        author_repository: Arc<dyn AuthorRepository + Send + Sync>,
        book_repository: Arc<dyn BookRepository + Send + Sync>,
        collection_repository: Arc<dyn CollectionRepository + Send + Sync>,
    ) -> Self {
        Self {
            author_repository,
            book_repository,
            collection_repository,
        }
    }
}
