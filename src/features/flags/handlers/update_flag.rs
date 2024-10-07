use rocket::{http::Status, response::status, State};
use serde_json::json;

use crate::core::AppState;
use crate::core::{
    responses,
    types::{ApiResponse, JsonResult},
};
use crate::features::flags;

pub async fn update_flag_handler(
    flag: &str,
    payload: JsonResult<'_, flags::schemas::FlagPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    // Flags are case insensitive
    let flag = flag.to_lowercase();

    let mut tx = state.pool.begin().await.unwrap();

    let app_flag = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE name = $1 LIMIT 1;",
        flag,
    )
    .fetch_optional(&mut *tx)
    .await
    .unwrap();

    if app_flag.is_none() {
        return Err(status::Custom(
            Status::NotFound,
            responses::error_response("Flag does not exist!"),
        ));
    }

    let app_flag = app_flag.unwrap();

    if app_flag.protected {
        return Err(status::Custom(
            Status::Forbidden,
            responses::error_response("Flag is protected!"),
        ));
    }

    if app_flag.internal {
        return Err(status::Custom(
            Status::Forbidden,
            responses::error_response("Internal flags are only modifiable by admins"),
        ));
    }

    if app_flag.value == payload.value {
        return Ok(json!(app_flag));
    }

    let new_flag = sqlx::query_as!(
        flags::schemas::Flag,
        "UPDATE flags SET value = $1 WHERE name = $2 RETURNING *;",
        payload.value,
        flag,
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    Ok(json!(new_flag))
}
