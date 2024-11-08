use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::core::AppState;
use crate::features::auth::guards::ExclusiveAuthDriver;
use crate::features::bus_driver;
use crate::features::bus_driver::schemas::BurritoRecordPayload;
use crate::router;

router!(DriverRouter, [post_driver_status]);

#[utoipa::path(
    request_body(content = BurritoRecordPayload),
    params(
        ("x-bus-id" = String, Header, description = "Unique bus driver identifier", example = "burrito-001"),
    ),
    security(("driver_auth" = [])),
    responses(
        (status = 200, description = "Driver status updated successfully"),
        (status = 401, description = "Unauthorized"),
    )
)]
#[post("/", format = "json", data = "<message_json>")]
async fn post_driver_status(
    driver: ExclusiveAuthDriver,
    message_json: Json<BurritoRecordPayload>,
    state: &State<AppState>,
) -> Status {
    let payload = message_json.into_inner();

    // Payload is completely delegated to the bus driver message handler.
    // Doing this also notifies the ws subscribers about the new position
    let _ = bus_driver::handlers::driver_message_handler(payload, state).await;

    // And of course, we must release the exclusive lock when the driver disconnects
    // In this case that's inmediately.
    state.drivers_locks.lock().await.remove(&driver.bus_name);

    Status::Ok
}
