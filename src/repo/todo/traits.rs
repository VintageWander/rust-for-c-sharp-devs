use async_trait::async_trait;
use uuid::Uuid;

use crate::{model::todo::Todo, Result};

#[async_trait]
pub trait ITodoRepository: Send + Sync + 'static {
    async fn create_todo(&self, item: Todo) -> Result<Todo>;
    async fn get_todo(&self, id: Uuid) -> Result<Todo>;
    async fn get_todos(&self) -> Result<Vec<Todo>>;
    async fn exists_todo(&self, name: &str) -> Result<bool>;
    async fn update_todo(&self, id: Uuid, item: Todo) -> Result<Todo>;
    async fn delete_todo(&self, id: Uuid) -> Result<()>;
}
