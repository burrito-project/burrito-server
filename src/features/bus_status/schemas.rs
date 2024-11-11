use serde::Serialize;
use utoipa::ToSchema;

use crate::features::bus_driver::schemas::BurritoPosRecord;
use crate::features::bus_stops::schemas::BusStopInfo;

#[derive(Serialize, ToSchema)]
pub struct BurritoStatusResponse {
    pub positions: Vec<BurritoPosRecord>,
    pub last_stop: Option<BusStopInfo>,
}
