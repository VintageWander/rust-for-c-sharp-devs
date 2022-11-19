use serde::{Deserialize, Serialize};

use crate::model::todo::Todo;
use crate::Result;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_finished: Option<String>,
}

impl UpdateTodoRequest {
    pub fn into_todo(self, old_todo: Todo) -> Result<Todo> {
        let is_finished = match self.is_finished {
            Some(finished) => match finished.as_str() {
                "true" => true,
                "false" => false,
                _ => return Err("Invalid finish format".into()),
            },
            None => old_todo.is_finished,
        };

        Ok(Todo {
            id: old_todo.id,
            name: self.name.unwrap_or(old_todo.name),
            description: self.description.unwrap_or(old_todo.description),
            is_finished,
        })
    }
}
