use rocket::{http::Status, serde::json::Json, Route, State};

use crate::features::auth;
use crate::{core::types::ApiResponse, entities::AppState};

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
