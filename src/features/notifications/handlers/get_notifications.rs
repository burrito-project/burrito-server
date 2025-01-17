use rocket::State;

use crate::core::AppState;
use crate::features::flags;
use crate::features::notifications::schemas;

pub async fn get_notifications_handler(state: &State<AppState>) -> Vec<schemas::Notification> {
    let random_order = flags::get_flag(&state.pool, "ads_random_order", true).await;

    match random_order {
        true => sqlx::query_as!(
            schemas::Notification,
            "SELECT * FROM notification_ads ORDER BY RANDOM();",
        )
        .fetch_all(&state.pool)
        .await
        .unwrap(),
        false => sqlx::query_as!(
            schemas::Notification,
            "SELECT * FROM notification_ads ORDER BY priority ASC;",
        )
        .fetch_all(&state.pool)
        .await
        .unwrap(),
    }
}
