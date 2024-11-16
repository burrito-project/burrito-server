use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::BurritoAPIError;
use crate::core::types::{ApiResponse, JsonResult};
use crate::core::AppState;
use crate::features::auth::guards::StaffUser;
use crate::features::updates::{self, schemas};
use crate::{docs, router};

router!(
    VersionsRouter,
    [
        list_app_versions,
        post_app_versions,
        patch_app_version,
        delete_app_version
    ]
);

#[utoipa::path(
    tag = docs::tags::APP_VERSIONS_TAG,
    description =
        "List all app versions history. Note that these are the official client versions, not the
        server versions. This list is not necessarily in sync with the actual app builds.",
    responses(
        (status = 200, body = Vec<schemas::AppVersion>),
    ),
)]
#[get("/")]
async fn list_app_versions(state: &State<AppState>) -> Json<Vec<schemas::AppVersion>> {
    Json(updates::handlers::list_app_versions_handler(state).await)
}

#[utoipa::path(
    description =
        "Creates a new app version. Should be used when a new APK is uploaded to the store.
        The version code should be the same as the one in the pubspec.yaml file.",
    tag = docs::tags::APP_VERSIONS_TAG,
    request_body(content = schemas::AppVersionPayload),
    responses(
        (status = 200, body = schemas::AppVersion),
        (status = 400),
        (status = 401),
    ),
    security(("staff_user_auth" = [])),
)]
#[post("/", format = "json", data = "<payload>")]
async fn post_app_versions(
    _user: StaffUser,
    payload: JsonResult<'_, schemas::AppVersionPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::AppVersion>> {
    if let Err(e) = payload {
        return BurritoAPIError::bad_request(None, e.to_string().into());
    }

    let payload = payload.unwrap().into_inner();

    let new_version = updates::handlers::post_app_version_handler(payload, state).await;

    Ok(Json(new_version))
}

#[utoipa::path(
    tag = docs::tags::APP_VERSIONS_TAG,
    description = "Edits an existing app version. All columns are optional.",
    request_body(content = schemas::AppVersionPatchPayload),
    responses(
        (status = 200, body = schemas::AppVersion),
        (status = 400),
        (status = 401),
    ),
    security(("staff_user_auth" = [])),
)]
#[patch("/<id>", format = "json", data = "<payload>")]
async fn patch_app_version(
    id: i32,
    _user: StaffUser,
    payload: JsonResult<'_, schemas::AppVersionPatchPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::AppVersion>> {
    if let Err(e) = payload {
        return BurritoAPIError::bad_request(None, e.to_string().into());
    }
    let payload = payload.unwrap().into_inner();

    updates::handlers::patch_app_version_handler(id, payload, state).await
}

#[utoipa::path(
    tag = docs::tags::APP_VERSIONS_TAG,
    description = "Deletes an existing app version.",
    responses(
        (status = 200, body = schemas::AppVersion),
        (status = 401),
    ),
    security(("staff_user_auth" = [])),
)]
#[delete("/<id>")]
async fn delete_app_version(
    id: i32,
    _user: StaffUser,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::AppVersion>> {
    updates::handlers::delete_app_version_handler(id, state).await
}
