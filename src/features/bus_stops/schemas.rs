use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::utils::{get_distance_to_bus_stop, get_next_bus_stop};

/// Latitude and longitude coordinates
pub struct LatLng {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
}

impl LatLng {
    pub const fn new(lat: f64, lng: f64) -> Self {
        LatLng { lat, lng }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// Represents a UNMSM bus stop
pub struct BusStopInfo {
    /// The stop popular name
    pub name: String,
    /// The stop number, as defined by the UNMSM
    pub number: i32,
    /// Whether the driver has reached this stop
    pub has_reached: bool,
    /// The time when this information was last updated
    pub timestamp: SystemTime,
    /// The distance, in meters, to reach this stop. Meaningless if has_reached is true
    pub distance: f64,
}

impl BusStopInfo {
    /// Logic for replacing the current last_stop information when a new position is received
    pub fn for_new_position(&self, new_pos: LatLng) -> Self {
        // If the last state is marked as reached, then we already passed it
        // and the bus is on its way to the next stop
        if self.has_reached {
            get_next_bus_stop(self, new_pos)
        }
        // And if not, we just update the distance to reach the bus stop
        else {
            let new_distance = get_distance_to_bus_stop(self, new_pos);
            BusStopInfo {
                distance: new_distance,
                has_reached: self.has_reached,
                name: self.name.to_owned(),
                number: self.number,
                timestamp: std::time::SystemTime::now(),
            }
        }
    }
}
