use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::{responses, AppState};
use crate::features::notifications::schemas;

pub async fn delete_notification_handler(id: i32, state: &State<AppState>) -> ApiResponse {
    let deleted_notification = sqlx::query_as!(
        schemas::Notification,
        "DELETE FROM notification_ads WHERE id = $1 RETURNING *;",
        id,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    if deleted_notification.is_none() {
        return Err(status::Custom(
            Status::NotFound,
            responses::error_response("Notification not found".to_string()),
        ));
    }

    Ok(json!(deleted_notification))
}
