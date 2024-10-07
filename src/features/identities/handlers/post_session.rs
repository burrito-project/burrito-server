use rocket::State;
use serde_json::json;
use sqlx::types::ipnetwork::IpNetwork;

use crate::core::AppState;
use crate::core::{guards::ForwardedIp, types::ApiResponse};
use crate::features::identities::schemas;

pub async fn post_session_handler(
    remote_addr: ForwardedIp,
    payload: schemas::UserIdentityPayload,
    state: &State<AppState>,
) -> ApiResponse {
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
