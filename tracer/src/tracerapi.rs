use core::fmt;
use poem::Result;
use poem_openapi::{OpenApi, payload::Json};
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
    // /// Rexx-Events
    // ///
    // /// Gibt alle Ereignisse aus einem bestimmten Kalender zurück von rexx
    // #[oai(path = "/rexx/events", method = "post")]
    // async fn users_rexx(
    //     &self,
    //     _auth: ApiAuthorization,
    //     rexx_url: Json<RexxUrlRequest>,
    // ) -> Result<Json<Vec<RexxEvent>>> {
    //     Ok(Json(events))
    // }
}
