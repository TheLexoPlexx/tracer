use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WsResponse {
    action: String,
    data: Option<String>,
    error: Option<Error>,
}

impl WsResponse {
    pub fn new(action: String) -> Self {
        Self {
            action,
            data: None,
            error: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Error {
    error_code: u16,
    message: String,
}
