use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::entities::AppState;
use crate::features::notifications::schemas;

pub async fn delete_notification_handler(id: i32, state: &State<AppState>) -> ApiResponse {
    let deleted_notification = sqlx::query_as!(
        schemas::Notification,
        "DELETE FROM notification_ads WHERE id = $1 RETURNING *;",
        id,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(json!(deleted_notification))
}
