use std::time::Duration;
use geo::GeodesicDistance;
use rocket::{http::Status, Route};
use crate::Message;
use rocket::State;
use rocket::serde::json::{json, Value};

use crate::BurritoState;

pub fn routes() -> Vec<Route> {
    routes![get_velocity]
}

pub fn calculate_velocity_kmph(positions: &[Message]) -> f64 {
    if positions.len() < 2 {
        return 0.0;
    }

    // meters
    let mut total_distance = 0.0;
    let mut total_time = Duration::new(0, 0);

    // we only use the last 5 positions to calculate the velocity
    let start = std::cmp::max(positions.len().saturating_sub(5), 1);

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

    if total_time.as_secs() == 0 {
        return 0.0;
    }

    // km/h
    let velocity = total_distance / total_time.as_secs() as f64;
    velocity * 3.6
}

#[get("/get-velocity")]
fn get_velocity(state: &State<BurritoState>) -> Result<Value, Status> {
    let messages = state.messages.lock().unwrap();
    if messages.is_empty() {
        return Err(Status::InternalServerError);
    }

    let velocity = calculate_velocity_kmph(&messages);
    Ok(json!({
        "velocity": velocity,
    }))
}
