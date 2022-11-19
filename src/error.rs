use salvo::{prelude::StatusError, Piece};
use thiserror::Error;

use crate::model::response::web::Web;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Var error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("Http Parse error: {0}")]
    Parse(#[from] salvo::http::ParseError),

    #[error("dotenv error: {0}")]
    Dotenv(#[from] dotenv::Error),

    #[error("Uuid error: {0}")]
    Uuid(#[from] uuid::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Generic(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Generic(s.to_string())
    }
}

impl Piece for Error {
    fn render(self, res: &mut salvo::Response) {
        let message = match self {
            Error::Generic(ref e) => format!("Generic error: {e}"),
            Error::Var(ref e) => format!("VarError: {e}"),
            Error::MongoDB(ref e) => format!("MongoDB error: {e}"),
            Error::Parse(ref e) => format!("Parse error: {e}"),
            Error::Dotenv(ref e) => format!("Dotenv error: {e}"),
            Error::Uuid(ref e) => format!("Uuid error: {e}"),
        };

        let status_error = match self {
            Error::Generic(_) => StatusError::conflict(),
            Error::Parse(_) | Error::MongoDB(_) | Error::Uuid(_) => StatusError::bad_request(),
            _ => StatusError::internal_server_error(),
        };

        let error = match self {
            Error::Generic(_) => Web::conflict(&message),
            Error::Parse(_) | Error::MongoDB(_) | Error::Uuid(_) => Web::bad_request(&message),
            _ => Web::internal_server_error(&message),
        };

        res.render(error);
        res.set_status_error(status_error)
    }
}
