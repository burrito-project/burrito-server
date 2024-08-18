use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::service_state::BusServiceState;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BurritoStateRecord {
    pub lt: f64,
    pub lg: f64,
    pub sts: BusServiceState,
    pub timestamp: SystemTime,
    pub velocity: f64,
}

/// The status payload received from the server, which contains the latitude, longitude
/// and status of the burrito
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BurritoRecordPayload {
    pub lt: f64,
    pub lg: f64,
    pub sts: BusServiceState, // i32
}
