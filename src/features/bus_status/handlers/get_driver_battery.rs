use rocket::State;

use crate::core::AppState;

pub async fn get_driver_battery_handler(state: &State<AppState>) -> Option<i32> {
    // The last record contains the last battery status sent
    let records = state.records.read().await;

    match records.last() {
        Some(last) => last.bat,
        None => None,
    }
}
