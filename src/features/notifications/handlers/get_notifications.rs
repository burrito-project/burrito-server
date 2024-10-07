use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::flags;
use crate::features::notifications::schemas;

pub async fn get_notifications_handler(state: &State<AppState>) -> ApiResponse {
    let random_order = flags::get_flag(&state.pool, "ads_random_order", true).await;

    match random_order {
        true => {
            let notifications = sqlx::query_as!(
                schemas::Notification,
                "SELECT * FROM notification_ads ORDER BY RANDOM();",
            )
            .fetch_all(&state.pool)
            .await
            .unwrap();

            Ok(json!(notifications))
        }
        false => {
            let notifications = sqlx::query_as!(
                schemas::Notification,
                "SELECT * FROM notification_ads ORDER BY priority ASC;",
            )
            .fetch_all(&state.pool)
            .await
            .unwrap();

            Ok(json!(notifications))
        }
    }
}
