use rocket::serde::json::Json;
use rocket::{http::Status, State};

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::updates;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_pending_updates, options]
}

#[get("/?<version>&<platform>")]
async fn get_pending_updates(
    version: Option<String>,
    platform: Option<String>,
    state: &State<AppState>,
) -> ApiResponse<Json<updates::schemas::PendingUpdatesResponse>> {
    updates::handlers::get_pending_updates_handler(version, platform, state).await
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
