use rocket::{http::Status, response::status, serde::json, serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::{core::responses, entities::AppState, features::flags};

pub fn routes() -> Vec<Route> {
    routes![list_all_flags, get_flag, modify_flag, options,]
}

#[get("/")]
async fn list_all_flags(state: &State<AppState>) -> Result<Value, status::Custom<Value>> {
    let flags = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE internal = false ORDER BY name ASC;",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("No notifications found"),
        )
    })?;

    Ok(json!(flags))
}

#[get("/<flag>")]
async fn get_flag(flag: &str, state: &State<AppState>) -> Result<Value, status::Custom<Value>> {
    let flag = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE name = $1 AND internal = false LIMIT 1;",
        flag,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("Failed to fetch flag"),
        )
    })?;

    Ok(json!(flag.value))
}

// TODO: create an admin endpoint to precede this, which will be allowed to create flags
// TODO: we are using too map_err garbage, we should create a responder to avoid this
#[put("/<flag>", format = "json", data = "<payload>")]
async fn modify_flag(
    flag: &str,
    payload: Result<Json<flags::schemas::FlagPayload>, json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    // Flags are case insensitive
    let flag = flag.to_lowercase();

    let mut tx = state.pool.begin().await.map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("Failed to start transaction"),
        )
    })?;

    let app_flag = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE name = $1 LIMIT 1;",
        flag,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("Failed to fetch flag"),
        )
    })?;

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
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) => status::Custom(
            Status::BadRequest,
            responses::error_response(db_err.to_string()),
        ),
        e => status::Custom(
            Status::InternalServerError,
            responses::error_response(format!("Failed to create notification: {}", e)),
        ),
    })?;

    tx.commit().await.map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("Failed to commit transaction"),
        )
    })?;

    Ok(json!(new_flag))
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
