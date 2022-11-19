use salvo::{affix, Router};

use crate::{
    handler::todo::{
        create_todo_handler, delete_todo_handler, get_todo_handler, get_todos_handler,
        update_todo_handler,
    },
    service::todo::TodoService,
};

pub struct TodoController {
    todo_service: TodoService,
}

impl TodoController {
    pub fn init(service: TodoService) -> Self {
        Self {
            todo_service: service,
        }
    }

    pub fn routes(&self) -> Router {
        Router::with_hoop(affix::insert("todo_service", self.todo_service.clone()))
            .push(Self::get_todos_route())
            .push(Self::create_todo_route())
            .push(Self::update_todo_route())
            .push(Self::delete_todo_route())
            .push(Self::get_todo_route())
    }

    fn get_todos_route() -> Router {
        Router::new().get(get_todos_handler)
    }

    fn get_todo_route() -> Router {
        Router::new().path("<param_todo_id>").get(get_todo_handler)
    }

    fn create_todo_route() -> Router {
        Router::with_path("create").post(create_todo_handler)
    }

    fn update_todo_route() -> Router {
        Router::with_path("update")
            .path("<param_todo_id>")
            .put(update_todo_handler)
    }

    fn delete_todo_route() -> Router {
        Router::with_path("delete")
            .path("<param_todo_id>")
            .delete(delete_todo_handler)
    }
}
