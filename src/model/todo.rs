use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_finished: bool,
}

impl Todo {
    pub fn new(name: String, description: String, is_finished: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            is_finished,
        }
    }
}

impl From<Todo> for Document {
    fn from(d: Todo) -> Self {
        doc! {
            "id": d.id,
            "name": d.name,
            "description": d.description,
            "isFinished": d.is_finished,
        }
    }
}
