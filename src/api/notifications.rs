use rocket::{http::Status, response::status, serde::json, serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::{core::responses, entities::AppState, schemas};

pub fn routes() -> Vec<Route> {
    routes![list_notifications, post_notifications,]
}

#[get("/")]
async fn list_notifications(state: &State<AppState>) -> Result<Value, status::Custom<Value>> {
    let notifications = sqlx::query_as!(
        schemas::Notification,
        "SELECT * FROM notification_ads ORDER BY priority ASC;",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("No notifications found"),
        )
    })?;

    Ok(json!(notifications))
}

#[post("/", format = "json", data = "<payload>")]
async fn post_notifications(
    payload: Result<Json<schemas::NotificationPayload>, json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    let new_notification = sqlx::query_as!(
        schemas::Notification,
        "INSERT INTO notification_ads
        (is_active, ad_title, ad_type, priority, image_url, target_url, ad_content, begin_at, end_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *;",
        payload.is_active,
        payload.ad_title,
        payload.ad_type.to_string(),
        payload.priority,
        payload.image_url,
        payload.target_url,
        payload.ad_content,
        payload.begin_at,
        payload.end_at,
    )
    .fetch_one(&state.pool)
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

    Ok(json!(new_notification))
}
