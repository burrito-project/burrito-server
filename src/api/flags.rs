use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::{ApiResponse, BurritoAPIError, JsonResult};
use crate::core::AppState;
use crate::docs;
use crate::features::{auth, flags};
use crate::router;

router!(FlagsRouter, [list_all_flags, get_flag, update_flag]);

#[utoipa::path(
    tag = docs::tags::FEATURE_FLAGS_TAG,
    responses(
        (status = 200, description = "Lists all the feature flags defined.", body = Vec<flags::schemas::Flag>),
    )
)]
#[get("/")]
async fn list_all_flags(state: &State<AppState>) -> Json<Vec<flags::schemas::Flag>> {
    let flags = flags::handlers::list_flags_handler(state).await;

    Json(flags)
}

#[utoipa::path(
    tag = docs::tags::FEATURE_FLAGS_TAG,
    responses(
        (status = 200, description = "Get a single feature flag by name.", body = flags::schemas::Flag),
    )
)]
#[get("/<flag>")]
async fn get_flag(flag: &str, state: &State<AppState>) -> ApiResponse<Json<flags::schemas::Flag>> {
    let flag = flags::handlers::get_flag_handler(flag, state).await;

    if flag.is_none() {
        return BurritoAPIError::not_found("Flag not found");
    }

    Ok(Json(flag.unwrap()))
}

#[utoipa::path(
    tag = docs::tags::FEATURE_FLAGS_TAG,
    request_body(content = flags::schemas::FlagPayload),
    security(("staff_user_auth" = [])),
    responses(
        (status = 200, description = "Update a feature flag by name.", body = flags::schemas::FlagPayload),
    )
)]
#[put("/<flag>", format = "json", data = "<payload>")]
async fn update_flag(
    flag: &str,
    _staff: auth::guards::StaffUser,
    payload: JsonResult<'_, flags::schemas::FlagPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<flags::schemas::Flag>> {
    flags::handlers::update_flag_handler(flag, payload, state).await
}
