use std::str::FromStr;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::Collection;
use uuid::Uuid;

use crate::Result;
use crate::{db::mongo::Mongo, model::todo::Todo};

use super::traits::ITodoRepository;

#[derive(Debug, Clone)]
pub struct TodoRepository {
    collection: Collection<Todo>,
}

impl TodoRepository {
    pub fn init(db: &Mongo) -> Self {
        Self {
            collection: db.get_collection("Todo"),
        }
    }
}

#[async_trait]
impl ITodoRepository for TodoRepository {
    async fn create_todo(&self, item: Todo) -> Result<Todo> {
        let todo_id = Uuid::from_str(&item.id)?;
        self.collection.insert_one(item, None).await?;
        let todo = self.get_todo(todo_id).await?;
        Ok(todo)
    }

    async fn get_todo(&self, id: Uuid) -> Result<Todo> {
        let todo = self
            .collection
            .find_one(doc! {"id": id.to_string()}, None)
            .await?
            .ok_or("Cannot find the task with the provided id")?;
        Ok(todo)
    }

    async fn get_todos(&self) -> Result<Vec<Todo>> {
        let todo = self
            .collection
            .find(None, None)
            .await?
            .try_collect()
            .await?;
        Ok(todo)
    }

    async fn exists_todo(&self, name: &str) -> Result<bool> {
        let count = self
            .collection
            .count_documents(doc! {"name": name}, None)
            .await?;
        Ok(count != 0)
    }

    async fn update_todo(&self, id: Uuid, item: Todo) -> Result<Todo> {
        let todo = self.get_todo(id).await?;

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let todo_doc: Document = item.into();

        let todo = self
            .collection
            .find_one_and_update(
                doc! {"id": id.to_string()},
                doc! {"$set": todo_doc},
                options,
            )
            .await?
            .ok_or("Cannot update task")?;
        Ok(todo)
    }

    async fn delete_todo(&self, id: Uuid) -> Result<()> {
        self.collection
            .delete_one(doc! {"id": id.to_string()}, None)
            .await?;
        Ok(())
    }
}
