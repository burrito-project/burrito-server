use geo::GeodesicDistance;
use rocket::Route;
use std::{iter, time::Duration};

use crate::entities::BurritoStateRecord;

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

        let time_diff = pos2
            .timestamp
            .duration_since(pos1.timestamp)
            .unwrap_or(Duration::new(0, 0));
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

#[allow(dead_code)]
pub fn get_uptime() -> std::time::Duration {
    let now = std::time::SystemTime::now();
    now.duration_since(*crate::startup).unwrap()
}

/// For prepending a base route to a Vec<Route>
pub fn with_base(
    routes: Vec<Route>,
    base: &'static str,
) -> iter::Map<impl Iterator<Item = Route>, impl FnMut(Route) -> Route> {
    routes.into_iter().map(move |route| {
        let route = route.clone();
        route
            .map_base(|base_| format!("{}/{}", base, base_))
            .unwrap()
    })
}
