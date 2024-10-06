use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::entities::AppState;
use crate::features::updates::schemas;

pub async fn post_app_version_handler(
    payload: schemas::AppVersionPayload,
    state: &State<AppState>,
) -> ApiResponse {
    let new_version = sqlx::query_as!(
        schemas::AppVersion,
        "INSERT INTO app_versions
        (semver, banner_url, is_mandatory, platform, release_date, release_notes)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *;",
        payload.semver,
        payload.banner_url,
        payload.is_mandatory,
        payload.platform.to_string(),
        payload.release_date,
        payload.release_notes,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(json!(new_version))
}
