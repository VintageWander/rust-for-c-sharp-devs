#![allow(dead_code, unused_variables)]

use dotenv::dotenv;

use controller::todo::TodoController;
use db::mongo::Mongo;
use error::Error;
use model::response::web::Web;
use repo::todo::impls::TodoRepository;
use salvo::{prelude::TcpListener, Server};
use service::todo::TodoService;

mod controller;
mod db;
mod error;
mod handler;
mod model;
mod repo;
mod service;
mod utils;

type Result<T> = std::result::Result<T, Error>;
type WebResult = Result<Web>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let db = Mongo::init().await?;
    let todo_repo = TodoRepository::init(&db);
    let todo_service = TodoService::init(todo_repo);
    let todo_controller = TodoController::init(todo_service);

    let router = todo_controller.routes();
    let port = std::env::var("PORT")?;
    let listener = TcpListener::bind(&format!("127.0.0.1:{port}"));
    Server::new(listener).serve(router).await;

    Ok(())
}
