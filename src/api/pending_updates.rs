use rocket::serde::json::Json;
use rocket::{http::Status, State};

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::updates::{self, schemas};
use crate::{docs, router};

router!(PendingUpdatesRouter, [get_pending_updates, options]);

#[utoipa::path(
    description =
        "The GET `/pending_updates?version=<semver>` endpoint will return a list of application
        versions that are newer than the one provided in the query parameter. This data should be
        consumed and verified on every app launch to ensure that the user is using the latest
        version of the app.",
    tag = docs::tags::APP_VERSIONS_TAG,
    params(
        ("version" = String, Query, description = "The current version of the app to check for new updates. Use semver format x.y.z", example = "1.0.0"),
        ("platform" = String, Query, description = "The platform of the app to check for new updates. Use android, ios, web, or all", example = "all"),
    ),
    responses(
        (status = 200, body = schemas::AppVersion, body = schemas::PendingUpdatesResponse),
        (status = 400),
        (status = 401),
    ),
)]
#[get("/?<version>&<platform>")]
async fn get_pending_updates(
    version: Option<String>,
    platform: Option<String>,
    state: &State<AppState>,
) -> ApiResponse<Json<updates::schemas::PendingUpdatesResponse>> {
    updates::handlers::get_pending_updates_handler(version, platform, state).await
}

#[utoipa::path(
    tag = docs::tags::APP_VERSIONS_TAG,
    responses((status = 200)),
)]
#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
