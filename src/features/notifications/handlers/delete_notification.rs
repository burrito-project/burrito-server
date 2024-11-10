use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::{ApiResponse, BurritoAPIError};
use crate::core::AppState;
use crate::features::notifications::schemas;

pub async fn delete_notification_handler(
    id: i32,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::Notification>> {
    let deleted_notification = sqlx::query_as!(
        schemas::Notification,
        "DELETE FROM notification_ads WHERE id = $1 RETURNING *;",
        id,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    if deleted_notification.is_none() {
        return BurritoAPIError::not_found("Notification not found");
    }

    Ok(Json(deleted_notification.unwrap()))
}
