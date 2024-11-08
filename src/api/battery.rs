use rocket::{Route, State};

use crate::core::AppState;
use crate::features::bus_status;
use crate::router;
use crate::{core::types::Json, routes::ApiRouter};

router!(BatteryRouter, [get_current_battery]);

#[derive(serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "battery": "69" }))]
struct BatteryResponse {
    battery: Option<i32>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Returns the bus driver app battery", body = BatteryResponse),
    )
)]
#[get("/")]
async fn get_current_battery(state: &State<AppState>) -> Json<BatteryResponse> {
    let battery = bus_status::handlers::get_driver_battery_handler(state).await;

    Json(BatteryResponse { battery })
}
