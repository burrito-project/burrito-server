use serde::{Deserialize, Serialize};

use crate::bus_stops::BusStopInfo;
use crate::entities::{BurritoStateRecord, BusServiceState};

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WsClientMessage {
    pub record: BurritoStateRecord,
    pub last_stop: Option<BusStopInfo>,
}

/// The status payload received from the server, which contains the latitude, longitude
/// and status of the burrito
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WsDriverMessage {
    pub lt: f64,
    pub lg: f64,
    pub sts: BusServiceState, // i32
}
