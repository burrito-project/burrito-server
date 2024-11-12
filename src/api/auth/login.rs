use rocket::{serde::json::Json, State};

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::auth;
use crate::{docs, router};

router!(AuthLoginRouter, [user_login]);

#[utoipa::path(
    tag = docs::tags::AUTH_TAG,
    description = "Login with a user account. Staff and root users only.",
    request_body(content = auth::schemas::UserLoginPayload),
    responses(
        (status = 200, body = auth::schemas::UserLoginResponse),
        (status = 401, description = "Invalid credentials... or something else?"),
    )
)]
#[post("/", format = "json", data = "<payload>")]
pub async fn user_login(
    payload: Json<auth::schemas::UserLoginPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<auth::schemas::UserLoginResponse>> {
    auth::handlers::user_login_handler(payload, state).await
}
