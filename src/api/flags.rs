use rocket::{http::Status, response::status, Route, State};
use serde_json::json;

use crate::core::{
    responses,
    types::{ApiResponse, JsonResult},
};
use crate::entities::AppState;
use crate::features::flags;

pub fn routes() -> Vec<Route> {
    routes![list_all_flags, get_flag, modify_flag, options,]
}

#[get("/")]
async fn list_all_flags(state: &State<AppState>) -> ApiResponse {
    let flags = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE internal = false ORDER BY name ASC;",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Ok(json!(flags))
}

#[get("/<flag>")]
async fn get_flag(flag: &str, state: &State<AppState>) -> ApiResponse {
    let flag = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE name = $1 AND internal = false LIMIT 1;",
        flag,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(json!(flag.value))
}

// TODO: create an admin endpoint to precede this, which will be allowed to create flags
// TODO: we are using too map_err garbage, we should create a responder to avoid this
#[put("/<flag>", format = "json", data = "<payload>")]
async fn modify_flag(
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

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
