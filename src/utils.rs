use salvo::{Depot, Request};
use serde::Deserialize;
use uuid::Uuid;

use crate::{service::todo::TodoService, Result};

pub fn get_todo_service(depot: &mut Depot) -> Result<&TodoService> {
    depot
        .get("todo_service")
        .ok_or_else(|| "Cannot get todo service".into())
}

pub fn get_param_todo_id(req: &mut Request) -> Result<Uuid> {
    req.param("param_todo_id")
        .ok_or_else(|| "Cannot get param todo id".into())
}

pub async fn extract_from_body<T: for<'a> Deserialize<'a>>(req: &mut Request) -> Result<T> {
    Ok(req.parse_body().await?)
}
