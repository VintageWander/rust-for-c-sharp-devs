use salvo::{handler, Depot, Request, Response};

use crate::{
    model::{request::todo::create::CreateTodoRequest, response::web::Web},
    utils::{extract_from_body, get_todo_service},
    WebResult,
};

#[handler]
pub async fn create_todo_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let todo_req = extract_from_body::<CreateTodoRequest>(req).await?;

    let todo_service = get_todo_service(depot)?;

    let todo = todo_service.create_todo(todo_req.into_todo()?).await?;

    Ok(Web::ok("Create task successfully", todo))
}
