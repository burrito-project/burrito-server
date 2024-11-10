use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::core::types::BurritoAPIError;
use crate::core::types::{ApiResponse, JsonResult};
use crate::core::AppState;
use crate::features::auth::guards::StaffUser;
use crate::features::updates::{self, schemas};

pub fn routes() -> Vec<Route> {
    routes![
        list_app_versions,
        post_app_versions,
        patch_app_version,
        delete_app_version,
    ]
}

#[get("/")]
async fn list_app_versions(state: &State<AppState>) -> Json<Vec<schemas::AppVersion>> {
    Json(updates::handlers::list_app_versions_handler(state).await)
}

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

#[delete("/<id>")]
async fn delete_app_version(
    id: i32,
    _user: StaffUser,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::AppVersion>> {
    updates::handlers::delete_app_version_handler(id, state).await
}
