use rocket::State;

use crate::core::types::ApiResponse;
use crate::core::AppState;

pub async fn get_driver_battery_handler(state: &State<AppState>) -> ApiResponse {
    let records = state.records.read().await;

    let battery: Option<i32> = match records.last() {
        Some(last) => last.bat,
        None => None,
    };

    Ok(serde_json::json!({
        "battery": battery
    }))
}
