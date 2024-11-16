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
    description = "Lists all the feature flags. Internal flags are omitted.",
    responses(
        (status = 200, body = Vec<flags::schemas::Flag>),
    )
)]
#[get("/")]
async fn list_all_flags(state: &State<AppState>) -> Json<Vec<flags::schemas::Flag>> {
    let flags = flags::handlers::list_flags_handler(state).await;

    Json(flags)
}

#[utoipa::path(
    tag = docs::tags::FEATURE_FLAGS_TAG,
    description =
        "Get a single feature flag by name. Flags are case insensitive.
        Internal flags will throw a 404.",
    responses(
        (status = 200, body = flags::schemas::Flag),
        (status = 404),
    )
)]
#[get("/<flag>")]
async fn get_flag(flag: &str, state: &State<AppState>) -> ApiResponse<Json<flags::schemas::Flag>> {
    // Note: To query internal flags use the features::flags::utils::get_flag function
    let flag = flags::handlers::get_flag_handler(flag, state).await;

    if flag.is_none() {
        return BurritoAPIError::not_found("Flag not found");
    }

    Ok(Json(flag.unwrap()))
}

#[utoipa::path(
    tag = docs::tags::FEATURE_FLAGS_TAG,
    description =
        "Update a feature flag by name. Flags are case insensitive. Staff authentication required.",
    request_body(content = flags::schemas::FlagPayload),
    security(("staff_user_auth" = [])),
    responses(
        (status = 200, body = flags::schemas::Flag),
        (status = 401),
        (status = 403, description = "Not enough privileges for the flag type"),
        (status = 404),
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
