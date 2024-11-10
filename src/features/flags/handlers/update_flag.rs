use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::BurritoAPIError;
use crate::core::types::{ApiResponse, JsonResult};
use crate::core::AppState;
use crate::features::flags;

pub async fn update_flag_handler(
    flag: &str,
    payload: JsonResult<'_, flags::schemas::FlagPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<flags::schemas::Flag>> {
    if let Err(e) = payload {
        return Err(BurritoAPIError::BadRequest {
            user_message: None,
            error: Some(e.to_string()),
        });
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
        return BurritoAPIError::not_found("Flag doesn't exist");
    }

    let app_flag = app_flag.unwrap();

    if app_flag.protected {
        return BurritoAPIError::forbbiden("Flag is protected!");
    }

    if app_flag.internal {
        return BurritoAPIError::forbbiden("Internal flags are only modifiable by admins");
    }

    if app_flag.value == payload.value {
        return Ok(Json(app_flag));
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

    Ok(Json(new_flag))
}
