use salvo::{prelude::StatusCode, writer::Json, Piece};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Web {
    code: String,
    message: String,
    data: Value,
    error: String,
}

impl Piece for Web {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(self));
        res.set_status_code(StatusCode::default())
    }
}

impl Web {
    pub fn ok<T: Serialize>(message: &str, data: T) -> Self {
        Self {
            code: StatusCode::OK.to_string(),
            message: message.to_string(),
            data: json!(data),
            error: String::default(),
        }
    }

    pub fn bad_request(error: &str) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST.to_string(),
            message: String::default(),
            data: json!(&()),
            error: error.to_string(),
        }
    }

    pub fn conflict(error: &str) -> Self {
        Self {
            code: StatusCode::CONFLICT.to_string(),
            message: String::default(),
            data: json!(&()),
            error: error.to_string(),
        }
    }

    pub fn internal_server_error(error: &str) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            message: String::default(),
            data: json!(&()),
            error: error.to_string(),
        }
    }
}
