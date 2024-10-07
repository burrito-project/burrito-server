use rocket::{http::Status, Route, State};

use crate::core::types::{ApiResponse, JsonResult};
use crate::core::AppState;
use crate::features::{auth, flags};

pub fn routes() -> Vec<Route> {
    routes![list_all_flags, get_flag, update_flag, options,]
}

#[get("/")]
async fn list_all_flags(state: &State<AppState>) -> ApiResponse {
    flags::handlers::list_flags_handler(state).await
}

#[get("/<flag>")]
async fn get_flag(flag: &str, state: &State<AppState>) -> ApiResponse {
    flags::handlers::get_flag_handler(flag, state).await
}

#[put("/<flag>", format = "json", data = "<payload>")]
async fn update_flag(
    flag: &str,
    _staff: auth::guards::StaffUser,
    payload: JsonResult<'_, flags::schemas::FlagPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    flags::handlers::update_flag_handler(flag, payload, state).await
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
