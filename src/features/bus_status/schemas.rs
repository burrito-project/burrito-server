use serde::Serialize;

use crate::features::bus_driver::schemas::BurritoPosRecord;
use crate::features::bus_stops::schemas::BusStopInfo;

#[derive(Serialize)]
pub struct StatusResponse {
    pub positions: Vec<BurritoPosRecord>,
    pub last_stop: Option<BusStopInfo>,
}
