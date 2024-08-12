use std::time::Duration;
use geo::GeodesicDistance;

use crate::BurritoStateRecord;

pub fn calculate_velocity_kmph(positions: &[BurritoStateRecord]) -> f64 {
    if positions.len() < 2 {
        return 0.0;
    }

    // meters
    let mut total_distance = 0.0;
    let mut total_time = Duration::new(0, 0);

    // we only use the last 3 positions to calculate the velocity
    let start = std::cmp::max(positions.len().saturating_sub(3), 1);

    for i in start..positions.len() {
        let pos1 = &positions[i - 1];
        let pos2 = &positions[i];

        let lat1 = pos1.lt;
        let lon1 = pos1.lg;
        let lat2 = pos2.lt;
        let lon2 = pos2.lg;

        // meters
        let distance = geo::Point::new(lat1, lon1).geodesic_distance(&geo::Point::new(lat2, lon2));

        total_distance += distance;

        let time_diff = pos2.timestamp.unwrap().duration_since(pos1.timestamp.unwrap()).unwrap_or(Duration::new(0, 0));
        total_time += time_diff;
    }

    let total_time = total_time.as_secs_f64();
    if total_time == 0.0 {
        return 0.0;
    }

    // km/h
    let velocity = total_distance / total_time;
    velocity * 3.6
}
