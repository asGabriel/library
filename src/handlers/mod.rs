use std::sync::Arc;

use crate::repositories::authors::AuthorRepository;

pub struct Handler {
    author_repository: Arc<dyn AuthorRepository>,
}
