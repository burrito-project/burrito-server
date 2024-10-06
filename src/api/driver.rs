use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::entities::{AppState, BurritoRecordPayload};
use crate::features::auth::guards::ExclusiveAuthDriver;

pub fn routes() -> Vec<Route> {
    routes![post_driver_status]
}

#[post("/", format = "json", data = "<message_json>")]
async fn post_driver_status(
    message_json: Json<BurritoRecordPayload>,
    driver: ExclusiveAuthDriver,
    state: &State<AppState>,
) -> Status {
    let payload = message_json.into_inner();

    // Payload is completely delegated to the websocket endpoint handler.
    // Doing this also notifies the subscribers about the new position
    let _ = crate::api::ws::driver_message_impl(payload, state).await;

    // And of course, we must release the lock when the driver disconnects
    // In this case that's inmediately.
    state.drivers_locks.lock().await.remove(&driver.bus_name);

    Status::Ok
}
