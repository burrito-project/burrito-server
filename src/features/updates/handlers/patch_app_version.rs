use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::updates::schemas;

pub async fn patch_app_version_handler(
    id: i32,
    payload: schemas::AppVersionPatchPayload,
    state: &State<AppState>,
) -> ApiResponse {
    // we only set the fields that are not None
    let updated_version = sqlx::query_as!(
        schemas::AppVersion,
        "UPDATE app_versions
        SET semver = COALESCE($2, semver),
            banner_url = COALESCE($3, banner_url),
            is_mandatory = COALESCE($4, is_mandatory),
            should_notify = COALESCE($5, should_notify),
            platform = COALESCE($6, platform),
            release_date = COALESCE($7, release_date),
            release_notes = COALESCE($8, release_notes)
        WHERE id = $1
        RETURNING *;",
        id,
        payload.semver,
        payload.banner_url,
        payload.is_mandatory,
        payload.should_notify,
        payload.platform.map(|p| p.to_string()),
        payload.release_date,
        payload.release_notes,
    );

    let updated_version = updated_version.fetch_one(&state.pool).await.unwrap();

    Ok(json!(updated_version))
}
