use rocket::{
    http::Status,
    response::status,
    serde::json::{self, Json},
    Route, State,
};
use serde_json::{json, Value};
use sqlx::types::ipnetwork::IpNetwork;

use crate::{
    core::{guards::ForwardedIp, responses},
    entities::AppState,
    schemas,
};

pub fn routes() -> Vec<Route> {
    routes![post_notifications, options]
}

#[post("/", format = "json", data = "<payload>")]
pub async fn post_notifications(
    remote_addr: ForwardedIp,
    payload: Result<Json<schemas::UserIdentityPayload>, json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
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
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) => status::Custom(
            Status::BadRequest,
            responses::error_response(db_err.to_string()),
        ),
        e => status::Custom(
            Status::InternalServerError,
            responses::error_response(format!("Failed to create notification: {}", e)),
        ),
    })?;

    Ok(json!(new_notification))
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
