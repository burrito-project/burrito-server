use rocket::serde::json::Json;
use rocket::State;

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
use crate::{docs, router};

router!(SessionRouter, [post_session]);

#[utoipa::path(
    description =
        "Registers a new app user session. Clients should call this endpoint every time the
        app is opened. To find out how this data is used, please refer to our
        [privacy policy](https://github.com/burrito-project/public/blob/main/PRIVACY_POLICY.md).",
    request_body = schemas::UserIdentityPayload,
    tag = docs::tags::ANALYTICS_TAG,
    responses(
        (status = 200, body = schemas::UserIdentity),
    )
)]
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
