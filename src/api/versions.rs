use rocket::{http::Status, response::status, Route, State};

use crate::core::{
    responses,
    types::{ApiResponse, JsonResult},
};
use crate::core::AppState;
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
async fn list_app_versions(state: &State<AppState>) -> ApiResponse {
    updates::handlers::list_app_versions_handler(state).await
}

#[post("/", format = "json", data = "<payload>")]
async fn post_app_versions(
    payload: JsonResult<'_, schemas::AppVersionPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    updates::handlers::post_app_version_handler(payload, state).await
}

#[patch("/<id>", format = "json", data = "<payload>")]
async fn patch_app_version(
    id: i32,
    payload: JsonResult<'_, schemas::AppVersionPatchPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }
    let payload = payload.unwrap().into_inner();

    updates::handlers::patch_app_version_handler(id, payload, state).await
}

#[delete("/<id>")]
async fn delete_app_version(id: i32, state: &State<AppState>) -> ApiResponse {
    updates::handlers::delete_app_version_handler(id, state).await
}
