use core::fmt;
use poem_openapi::{OpenApi, payload::PlainText};
use std::error::Error;

#[derive(Debug)]
struct ApiError(String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ApiError {}

pub struct TracerApi;

#[OpenApi]
impl TracerApi {
    /// Ping
    ///
    /// Gibt "pong" zurück
    #[oai(path = "/ping", method = "get")]
    async fn ping(&self) -> PlainText<String> {
        PlainText("pong".to_string())
    }
}
