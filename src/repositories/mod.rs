pub mod authors;
pub mod books;
pub mod collections;
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct SqlxRepository {
    pool: PgPool,
}

impl SqlxRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
