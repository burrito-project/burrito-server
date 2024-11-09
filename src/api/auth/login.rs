use rocket::{http::Status, serde::json::Json, State};

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::auth;
use crate::{docs, router};

router!(AuthLoginRouter, [user_login, options]);

#[utoipa::path(
    tag = docs::tags::AUTH_TAG,
    description = "Login a user",
    request_body(content = auth::schemas::UserLoginPayload),
    responses(
        (status = 200, description = "Driver status updated successfully"),
        (status = 401, description = "Unauthorized"),
    )
)]
#[post("/", format = "json", data = "<payload>")]
pub async fn user_login(
    payload: Json<auth::schemas::UserLoginPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    auth::handlers::user_login_handler(payload, state).await
}

#[utoipa::path(
    tag = docs::tags::AUTH_TAG,
)]
#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
