use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::core::AppState;
use crate::docs;
use crate::features::auth::guards::ExclusiveAuthDriver;
use crate::features::bus_driver;
use crate::features::bus_driver::schemas::BurritoRecordPayload;
use crate::router;

router!(DriverRouter, [post_driver_status]);

#[utoipa::path(
    tag = docs::tags::BUS_DRIVER_TAG,
    description =
        "Endpoint for bus drivers to send their location and status data.
        \nThis data is stored in the form of \"records\" that can be later queried using the
        `/status` endpoint. In the case of WebSocket clients, the data is instantly broadcasted.",
    request_body(content = BurritoRecordPayload),
    params(
        (
            "x-bus-id" = String, Header,
            description = "Unique bus driver identifier. Aims to support multiple bus drivers at the same time.",
            example = "burrito-001",
        ),
    ),
    security(("driver_auth" = [])),
    responses(
        (status = 200),
        (status = 401),
        (status = 429, description = "There is a driver with the same `x-bus-id` already connected"),
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
