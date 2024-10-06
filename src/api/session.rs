use rocket::{http::Status, response::status, Route, State};

use crate::entities::AppState;
use crate::features::identities::schemas;
use crate::{
    core::{
        guards::ForwardedIp,
        responses,
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
) -> ApiResponse {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    identities::handlers::post_session_handler(remote_addr, payload, state).await
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
