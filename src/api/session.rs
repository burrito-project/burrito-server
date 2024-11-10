use rocket::serde::json::Json;
use rocket::{http::Status, Route, State};

use crate::core::types::BurritoAPIError;
use crate::core::AppState;
use crate::features::identities::schemas;
use crate::{
    core::{
        guards::ForwardedIp,
        types::{ApiResponse, JsonResult},
    },
    features::identities,
};

pub fn routes() -> Vec<Route> {
    routes![post_session, options]
}

#[post("/", format = "json", data = "<payload>")]
pub async fn post_session(
    remote_addr: ForwardedIp,
    payload: JsonResult<'_, schemas::UserIdentityPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::UserIdentity>> {
    if let Err(e) = payload {
        return BurritoAPIError::bad_request(None, e.to_string().into());
    }

    let payload = payload.unwrap().into_inner();

    Ok(Json(
        identities::handlers::post_session_handler(remote_addr, payload, state).await,
    ))
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
