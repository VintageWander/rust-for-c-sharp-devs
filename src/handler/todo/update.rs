use salvo::{handler, Depot, Request, Response};

use crate::{
    model::{request::todo::update::UpdateTodoRequest, response::web::Web},
    utils::{extract_from_body, get_param_todo_id, get_todo_service},
    WebResult,
};

#[handler]
pub async fn update_todo_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let todo_req = extract_from_body::<UpdateTodoRequest>(req).await?;

    let param_todo_id = get_param_todo_id(req)?;

    let todo_service = get_todo_service(depot)?;

    let old_todo = todo_service.get_todo(param_todo_id).await?;

    let todo = todo_service
        .update_todo(param_todo_id, todo_req.into_todo(old_todo)?)
        .await?;

    Ok(Web::ok("Update task successfully", todo))
}
