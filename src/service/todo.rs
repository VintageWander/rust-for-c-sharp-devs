use std::sync::Arc;

use uuid::Uuid;

use crate::{model::todo::Todo, repo::todo::traits::ITodoRepository, Result};

#[derive(Clone)]
pub struct TodoService {
    todo_repo: Arc<dyn ITodoRepository>,
}

impl TodoService {
    pub fn init(todo_repo: impl ITodoRepository) -> Self {
        Self {
            todo_repo: Arc::new(todo_repo),
        }
    }

    pub async fn exists_todo(&self, name: &str) -> Result<bool> {
        self.todo_repo.exists_todo(name).await
    }

    pub async fn get_todo(&self, id: Uuid) -> Result<Todo> {
        self.todo_repo.get_todo(id).await
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        self.todo_repo.get_todos().await
    }

    pub async fn create_todo(&self, item: Todo) -> Result<Todo> {
        match self.exists_todo(&item.name).await? {
            true => Err("This task is already existed. Please try another name".into()),
            false => self.todo_repo.create_todo(item).await,
        }
    }

    pub async fn update_todo(&self, id: Uuid, item: Todo) -> Result<Todo> {
        let old_item = self.get_todo(id).await?;
        match old_item.name != item.name && self.exists_todo(&item.name).await? {
            true => Err("This name has already taken. Please pick another name".into()),
            false => self.todo_repo.update_todo(id, item).await,
        }
    }

    pub async fn delete_todo(&self, id: Uuid) -> Result<()> {
        self.todo_repo.delete_todo(id).await
    }
}
