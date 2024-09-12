use rocket::http::Status;
use rocket::serde::json::Value;
use rocket::{Route, State};
use serde::Serialize;

use crate::bus_stops::BusStopInfo;
use crate::entities::{AppState, BurritoPosRecord};

pub fn routes() -> Vec<Route> {
    routes![get_current_battery]
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub positions: Vec<BurritoPosRecord>,
    pub last_stop: Option<BusStopInfo>,
}

#[get("/")]
async fn get_current_battery(state: &State<AppState>) -> Result<Value, Status> {
    let records = state.records.read().await;

    let battery: Option<i32> = match records.last() {
        Some(last) => last.bat,
        None => None,
    };

    Ok(serde_json::json!({
        "battery": battery
    }))
}
