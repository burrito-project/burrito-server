use rocket::serde::json::Json;
use rocket::State;

use crate::core::AppState;
use crate::docs;
use crate::features::bus_status;
use crate::router;

router!(BatteryRouter, [get_current_battery]);

#[derive(serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "battery": "69" }))]
struct BatteryResponse {
    battery: Option<i32>,
}

#[utoipa::path(
    tag = docs::tags::BUS_INFO_TAG,
    description =
        "Returns the last known bus driver app battery. If the bus has been idle for a while, a
        `null` value is returned.
        \n\nTo see the full battery history, use the `/status` endpoint instead.",
    responses(
        (status = 200, body = BatteryResponse),
    )
)]
#[get("/")]
async fn get_current_battery(state: &State<AppState>) -> Json<BatteryResponse> {
    let battery = bus_status::handlers::get_driver_battery_handler(state).await;

    Json(BatteryResponse { battery })
}
