use super::SqlxRepository;
use crate::domain::{
    authors::{Author, CreateAuthor, UpdateAuthor},
    errors::Result,
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait AuthorRepository {
    async fn create_author(&self, author: CreateAuthor) -> Result<Author>;
    async fn get_author_by_id(&self, author_id: Uuid) -> Result<Option<Author>>;
    async fn update_author_by_id(
        &self,
        author: UpdateAuthor,
        author_id: Uuid,
    ) -> Result<Option<Author>>;
}

#[async_trait::async_trait]
impl AuthorRepository for SqlxRepository {
    async fn create_author(&self, author: CreateAuthor) -> Result<Author> {
        let rec = sqlx::query_as!(
            Author,
            r#"
            INSERT INTO AUTHORS (AUTHOR_ID, NAME, DATE_OF_BIRTH)
            VALUES ( $1, $2, $3 )
            RETURNING *
            "#,
            Uuid::new_v4(),
            author.name,
            author.date_of_birth
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn get_author_by_id(&self, author_id: Uuid) -> Result<Option<Author>> {
        let rec = sqlx::query_as!(
            Author,
            r#"
            SELECT * FROM AUTHORS WHERE AUTHOR_ID = $1
            "#,
            author_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn update_author_by_id(
        &self,
        author: UpdateAuthor,
        author_id: Uuid,
    ) -> Result<Option<Author>> {
        let rec = sqlx::query_as!(
            Author,
            r#"
            UPDATE AUTHORS SET NAME=$1, DATE_OF_BIRTH=$2 WHERE AUTHOR_ID=$3
            RETURNING *
            "#,
            author.name,
            author.date_of_birth,
            author_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }
}
