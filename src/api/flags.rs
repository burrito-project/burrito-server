use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;

use crate::core::types::{ApiResponse, JsonResult};
use crate::core::{responses, AppState};
use crate::features::{auth, flags};
use crate::router;

router!(FlagsRouter, [list_all_flags, get_flag, update_flag]);

#[utoipa::path(
    tag = "Feature flags",
    responses(
        (status = 200, description = "Lists all the feature flags defined."),
    )
)]
#[get("/")]
async fn list_all_flags(state: &State<AppState>) -> Json<Vec<flags::schemas::Flag>> {
    let flags = flags::handlers::list_flags_handler(state).await;

    Json(flags)
}

#[utoipa::path(
    tag = "Feature flags",
    responses(
        (status = 200, description = "Get a single feature flag by name.", body = flags::schemas::Flag),
    )
)]
#[get("/<flag>")]
async fn get_flag(flag: &str, state: &State<AppState>) -> ApiResponse {
    let flag = flags::handlers::get_flag_handler(flag, state).await;

    if flag.is_none() {
        return Err(status::Custom(
            Status::NotFound,
            responses::error_response("Flag not found".to_string()),
        ));
    }

    Ok(json!(flag.unwrap()))
}

#[utoipa::path(
    tag = "Feature flags",
    request_body(content = flags::schemas::FlagPayload),
    security(("staff_user_auth" = [])),
    responses(
        (status = 200, description = "Update a feature flag by name."),
    )
)]
#[put("/<flag>", format = "json", data = "<payload>")]
async fn update_flag(
    flag: &str,
    _staff: auth::guards::StaffUser,
    payload: JsonResult<'_, flags::schemas::FlagPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    flags::handlers::update_flag_handler(flag, payload, state).await
}
