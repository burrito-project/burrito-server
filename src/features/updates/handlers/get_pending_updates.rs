use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::{ApiResponse, BurritoAPIError};
use crate::core::AppState;
use crate::features::updates::schemas::PendingUpdatesResponse;
use crate::features::updates::{schemas, utils};

pub async fn get_pending_updates_handler(
    version: Option<String>,
    platform: Option<String>,
    state: &State<AppState>,
) -> ApiResponse<Json<PendingUpdatesResponse>> {
    let user_version = match version {
        Some(semver) => {
            if !utils::is_valid_semver(&semver) {
                return BurritoAPIError::bad_request(
                    "Invalid version format. Use x.y.z".into(),
                    None,
                );
            }
            semver
        }
        None => {
            return BurritoAPIError::bad_request(
                "No version param provided. Use ?version=<x.y.z>".into(),
                None,
            );
        }
    };

    let user_platform: schemas::PlatformType = match platform {
        Some(p) => match schemas::PlatformType::try_from(p.as_str()) {
            Ok(p) => p,
            Err(_) => {
                return BurritoAPIError::bad_request(
                    "Invalid platform. Use ?platform=<android|ios|web|all>".into(),
                    None,
                );
            }
        },
        None => {
            return BurritoAPIError::bad_request(
                "No platform param provided. Use ?platform=<android|ios|web>".into(),
                None,
            );
        }
    };

    let app_versions = sqlx::query_as!(
        schemas::AppVersion,
        r#"SELECT *
        FROM app_versions
        WHERE semver > $1 AND (platform = 'all' OR platform = $2) AND should_notify = true
        ORDER BY (semver, release_date) DESC;"#, // first is newer
        user_version,
        user_platform.to_string(),
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    let must_update = app_versions.iter().any(|version| version.is_mandatory);

    Ok(Json(PendingUpdatesResponse {
        must_update,
        versions: app_versions
            .iter()
            .map(|v| schemas::PendingUpdate {
                semver: v.semver.clone(),
                banner_url: v.banner_url.clone().unwrap_or_default(),
                is_mandatory: v.is_mandatory,
                release_date: v.release_date.to_string(),
                release_notes: v.release_notes.clone().unwrap_or_default(),
            })
            .collect(),
    }))
}
