use rocket::{Route, State};

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::bus_status;

pub fn routes() -> Vec<Route> {
    routes![get_current_battery]
}

#[get("/")]
async fn get_current_battery(state: &State<AppState>) -> ApiResponse {
    bus_status::handlers::get_driver_battery_handler(state).await
}
