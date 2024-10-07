use rocket::State;
use serde_json::json;

use crate::core::guards::IsMobileChecker;
use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::flags;
use crate::features::notifications::schemas;

pub async fn get_notifications_handler(
    state: &State<AppState>,
    is_mobile: IsMobileChecker,
) -> ApiResponse {
    if is_mobile.ask() {
        // Empty notifications for mobile while we are on review
        // TODO: remove
        return Ok(json!([]));
    }

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
