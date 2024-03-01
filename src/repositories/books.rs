use uuid::Uuid;

use crate::domain::{
    books::{Book, CreateBook, Genre, Lang},
    errors::Result,
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait BookRepository {
    async fn create_book(&self, book: CreateBook) -> Result<Book>;
}

#[async_trait::async_trait]
impl BookRepository for SqlxRepository {
    async fn create_book(&self, book: CreateBook) -> Result<Book> {
        let new_book = sqlx::query_as!(
            Book,
            r#"
            INSERT INTO BOOKS (BOOK_ID, AUTHOR_ID, COLLECTION_ID, NAME, GENRE, LANG, RATING)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING BOOK_ID, AUTHOR_ID, COLLECTION_ID, NAME, genre as "genre: _", lang as "lang: _", RATING, CREATION_TIME, DELETION_TIME, UPDATED_AT
            "#,
            Uuid::new_v4(),
            book.author_id,
            book.collection_id,
            book.name,
            book.genre as Genre,
            book.lang as Lang,
            book.rating
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(new_book)
    }
}
