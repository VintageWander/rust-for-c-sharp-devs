use crate::utils::get_param_todo_id;
use salvo::{handler, Depot, Request, Response};

use crate::{model::response::web::Web, utils::get_todo_service, WebResult};

#[handler]
pub async fn get_todos_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let todo_service = get_todo_service(depot)?;

    let todos = todo_service.get_todos().await?;

    Ok(Web::ok("Get tasks successfully", todos))
}

#[handler]
pub async fn get_todo_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let todo_service = get_todo_service(depot)?;

    let param_todo_id = get_param_todo_id(req)?;

    let todo = todo_service.get_todo(param_todo_id).await?;

    Ok(Web::ok("Get tasks successfully", todo))
}
