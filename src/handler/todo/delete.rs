use salvo::{handler, Depot, Request, Response};

use crate::{
    model::response::web::Web,
    utils::{get_param_todo_id, get_todo_service},
    WebResult,
};

#[handler]
pub async fn delete_todo_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let param_todo_id = get_param_todo_id(req)?;

    let todo_service = get_todo_service(depot)?;

    todo_service.delete_todo(param_todo_id).await?;

    Ok(Web::ok("Delete task successfully", ()))
}
