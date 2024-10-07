use rocket::http::Status;
use rocket::{Route, State};

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::bus_status;

pub fn routes() -> Vec<Route> {
    routes![get_status, options]
}

const DEFAULT_COUNT: usize = 100;

#[get("/?<count>")]
async fn get_status(count: Option<usize>, state: &State<AppState>) -> ApiResponse {
    let count = count.unwrap_or(DEFAULT_COUNT);
    let status = bus_status::handlers::get_burrito_status_handler(count, state).await;

    Ok(serde_json::json!(status))
}

#[options("/")]
fn options() -> Status {
    Status::Ok
}
