use mockall::automock;
use uuid::Uuid;

use crate::domain::{
    books::{Book, CreateBook, Genre, Lang, UpdateBook},
    errors::Result,
};

use super::SqlxRepository;

#[automock]
#[async_trait::async_trait]
pub trait BookRepository {
    async fn create_book(&self, book: &CreateBook) -> Result<Book>;
    async fn get_book_by_id(&self, book_id: Uuid) -> Result<Option<Book>>;
    async fn list_all_books(&self) -> Result<Vec<Book>>;
    async fn update_book_by_id(&self, book_id: Uuid, book: UpdateBook) -> Result<Option<Book>>;
    async fn delete_book_by_id(&self, book_id: Uuid) -> Result<Option<Book>>;
}

#[async_trait::async_trait]
impl BookRepository for SqlxRepository {
    async fn create_book(&self, book: &CreateBook) -> Result<Book> {
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

    async fn get_book_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        let book: Option<Book> = sqlx::query_as!(
            Book,
            r#"
            SELECT BOOK_ID, AUTHOR_ID, COLLECTION_ID, NAME, genre as "genre: _", lang as "lang: _", RATING, CREATION_TIME, DELETION_TIME, UPDATED_AT 
            FROM BOOKS WHERE BOOK_ID = $1 AND DELETION_TIME IS NULL
            "#,
            book_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(book)
    }

    async fn list_all_books(&self) -> Result<Vec<Book>> {
        let books = sqlx::query_as!(
            Book,
            r#"
            SELECT 
                BOOK_ID, 
                AUTHOR_ID,
                COLLECTION_ID, 
                NAME,
                genre as "genre: _",
                lang as "lang: _",
                RATING,
                CREATION_TIME,
                DELETION_TIME,
                UPDATED_AT
            FROM 
                BOOKS 
            WHERE 
                DELETION_TIME IS NULL
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(books)
    }

    async fn update_book_by_id(&self, book_id: Uuid, book: UpdateBook) -> Result<Option<Book>> {
        let book = sqlx::query_as!(
            Book,
            r#"
            UPDATE BOOKS SET
                AUTHOR_ID = COALESCE($2, AUTHOR_ID),
                COLLECTION_ID = COALESCE($3, COLLECTION_ID), 
                NAME = COALESCE($4, NAME),
                GENRE = COALESCE($5, GENRE),
                LANG = COALESCE($6, LANG),
                RATING = COALESCE($7, RATING)
            WHERE
                BOOK_ID = $1 AND DELETION_TIME IS NULL
            RETURNING
                BOOK_ID, 
                AUTHOR_ID,
                COLLECTION_ID, 
                NAME,
                GENRE as "genre: _",
                LANG as "lang: _",
                RATING,
                CREATION_TIME,
                DELETION_TIME,
                UPDATED_AT
            "#,
            book_id,
            book.author_id,
            book.collection_id,
            book.name,
            book.genre as _,
            book.lang as _,
            book.rating
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(book)
    }

    async fn delete_book_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        let book = sqlx::query_as!(
            Book,
            r#"
            UPDATE BOOKS SET
                UPDATED_AT = NOW(),
                DELETION_TIME = NOW()
            WHERE
                BOOK_ID = $1 AND DELETION_TIME IS NULL
            RETURNING
                BOOK_ID, 
                AUTHOR_ID,
                COLLECTION_ID, 
                NAME,
                GENRE as "genre: _",
                LANG as "lang: _",
                RATING,
                CREATION_TIME,
                DELETION_TIME,
                UPDATED_AT
            "#,
            book_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(book)
    }
}
