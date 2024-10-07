use rocket::{http::Status, serde::json::Json, Route, State};

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::auth;

pub fn routes() -> Vec<Route> {
    routes![user_login, options]
}

#[post("/", format = "json", data = "<payload>")]
pub async fn user_login(
    payload: Json<auth::schemas::UserLoginPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    auth::handlers::user_login_handler(payload, state).await
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
