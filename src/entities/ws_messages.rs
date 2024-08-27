use serde::{Deserialize, Serialize};

use crate::bus_stops::BusStopInfo;
use crate::entities::BurritoPosRecord;

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WsClientMessage {
    pub record: BurritoPosRecord,
    pub last_stop: Option<BusStopInfo>,
}
