use rocket::{http::Status, response::status, Route, State};
use serde_json::json;
use sqlx::types::ipnetwork::IpNetwork;

use crate::core::{
    guards::ForwardedIp,
    responses,
    types::{ApiResponse, JsonResult},
};
use crate::entities::AppState;
use crate::schemas;

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
    let user_ip = IpNetwork::new(remote_addr.into(), 32).unwrap();

    let new_notification = sqlx::query_as!(
        schemas::UserIdentity,
        "INSERT INTO user_identities (fingerprint, last_ip, last_version, platform, session_count)
        VALUES ($1, $2, $3, $4, 1)
        ON CONFLICT (fingerprint)
        DO UPDATE SET
            last_ip = EXCLUDED.last_ip,
            last_version = EXCLUDED.last_version,
            platform = EXCLUDED.platform,
            session_count = user_identities.session_count + 1
        RETURNING *;",
        payload.fingerprint,
        user_ip,
        payload.last_version,
        payload.platform,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(json!(new_notification))
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
