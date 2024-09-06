use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::service_state::BusServiceState;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BurritoPosRecord {
    pub lt: f64,
    pub lg: f64,
    pub sts: BusServiceState,
    pub timestamp: SystemTime,
    pub velocity: f64,
}

impl BurritoPosRecord {
    pub fn formatted_time_ago(&self) -> String {
        let elapsed = self.timestamp.elapsed().unwrap();
        let secs = elapsed.as_secs();
        let mins = secs / 60;
        let hours = mins / 60;
        let days = hours / 24;

        if days > 0 {
            format!("hace {} días", days)
        } else if hours > 0 {
            format!("hace {} horas", hours)
        } else if mins > 0 {
            format!("hace {} minutos", mins)
        } else if secs == 0 {
            "justo ahora".to_string()
        } else {
            format!("hace {} segundos", secs)
        }
    }
}

/// The status payload received from the server, which contains the latitude, longitude
/// and status of the burrito
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BurritoRecordPayload {
    pub lt: f64,
    pub lg: f64,
    pub sts: BusServiceState, // i32
}
