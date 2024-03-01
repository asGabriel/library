use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Databse error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Author not found")]
    AuthorNotFound(Uuid),
    #[error("Book not found")]
    BookNotFound(Uuid),
}

pub type Result<T> = std::result::Result<T, Error>;
