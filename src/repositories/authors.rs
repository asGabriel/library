use super::SqlxRepository;
use crate::domain::{
    authors::{Author, CreateAuthor, UpdateAuthor},
    errors::Result,
};
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait::async_trait]
pub trait AuthorRepository {
    async fn create_author(&self, author: CreateAuthor) -> Result<Author>;
    async fn get_author_by_id(&self, author_id: Uuid) -> Result<Option<Author>>;
    async fn update_author_by_id(
        &self,
        author: UpdateAuthor,
        author_id: Uuid,
    ) -> Result<Option<Author>>;
    async fn delete_author_by_id(&self, author_id: Uuid) -> Result<Option<Author>>;
    async fn list_authors(&self) -> Result<Vec<Author>>;
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
            SELECT * FROM AUTHORS WHERE AUTHOR_ID = $1 AND DELETION_TIME IS NULL
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
            UPDATE AUTHORS SET NAME=COALESCE($1,NAME), DATE_OF_BIRTH=COALESCE($2,DATE_OF_BIRTH), UPDATED_AT=NOW() 
            WHERE AUTHOR_ID=$3 AND DELETION_TIME IS NULL
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

    async fn delete_author_by_id(&self, author_id: Uuid) -> Result<Option<Author>> {
        let rec = sqlx::query_as!(
            Author,
            r#"
            UPDATE AUTHORS SET DELETION_TIME=NOW() WHERE AUTHOR_ID=$1 AND DELETION_TIME IS NULL
            RETURNING *
            "#,
            author_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn list_authors(&self) -> Result<Vec<Author>> {
        let authors = sqlx::query_as!(
            Author,
            r#"
            SELECT 
                *
            FROM 
                AUTHORS 
            WHERE 
                DELETION_TIME IS NULL
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(authors)
    }
}
