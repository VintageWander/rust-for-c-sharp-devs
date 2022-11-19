use serde::{Deserialize, Serialize};

use crate::model::todo::Todo;
use crate::Result;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodoRequest {
    pub name: String,
    pub description: String,
    pub is_finished: String,
}

impl CreateTodoRequest {
    pub fn into_todo(self) -> Result<Todo> {
        let is_finished = match self.is_finished.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err("Invalid finish format".into()),
        };

        Ok(Todo::new(
            self.name.to_string(),
            self.description,
            is_finished,
        ))
    }
}
