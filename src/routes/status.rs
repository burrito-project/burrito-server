use std::time::SystemTime;
use rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::utils;
use crate::AppState;
use crate::BurritoStateRecord;
use crate::entities::service_state::BusServiceState;
use crate::bus_stops::{get_bus_stop_for_point, get_distance_to_bus_stop, get_next_bus_stop, LatLng};

pub fn routes() -> Vec<Route> {
    routes![get_status, post_status]
}

const DEFAULT_COUNT: usize = 100;

#[get("/?<count>")]
fn get_status(count: Option<usize>, state: &State<AppState>) -> Result<Value, Status> {
    let count = count.unwrap_or(DEFAULT_COUNT);

    let messages = state.messages.read().unwrap();
    let last_stop = state.last_stop.read().unwrap();

    let n = std::cmp::min(count, messages.len());

    match messages.last() {
        Some(last) => {
            let is_off = last.sts == 1 || last.sts == 2 || last.sts == 3;

            // If the burrito didn't report itself as 1,2 or 3 and it hasn't reported in the last 60 seconds,
            // then we consider it as off
            if !is_off && last.timestamp.unwrap().elapsed().unwrap() > std::time::Duration::from_secs(60) {
                // We create an 'off' message on the fly
                let off_message = BurritoStateRecord {
                    lt: 0.0,
                    lg: 0.0,
                    sts: BusServiceState::Off.into(),
                    timestamp: last.timestamp,
                    velocity: 0.0,
                };

                let mut messages_cpy = messages.clone();
                messages_cpy.push(off_message);
                *state.last_stop.write().unwrap() = None;

                return Ok(json!({
                    "positions": messages_cpy.iter().rev().take(n).cloned().collect::<Vec<BurritoStateRecord>>(),
                    "last_stop": null,
                }));
            }
        },
        None => {
            return Ok(json!({
                "positions": vec![BurritoStateRecord {
                    lt: 0.0,
                    lg: 0.0,
                    sts: BusServiceState::Off.into(),
                    timestamp: Some(SystemTime::now()),
                    velocity: 0.0,
                }],
                "last_stop": last_stop.clone(),
            }));
        },
    }

    let recent_messages: Vec<BurritoStateRecord> = messages.iter().rev().take(n).cloned().collect();

    Ok(json!({
        "positions": recent_messages,
        "last_stop": last_stop.clone(),
    }))
}

#[post("/", format = "json", data = "<message_json>")]
fn post_status(message_json: Json<BurritoStateRecord>, state: &State<AppState>) -> Status {
    let messages = state.messages.read().unwrap();
    let mut message = message_json.into_inner();

    match get_bus_stop_for_point(message.lt, message.lg) {
        Some(this_stop) => {
            let mut last_stop = state.last_stop.write().unwrap();
            // If there's already last_stop we update it
            *last_stop = Some(this_stop);
        },
        None => {
            // If the burrito is not in a bus stop and we have a last_stop (has_reached=true),
            // we interpret as it has left that bus stop, so we choose the next one as has_reached=false
            let mut last_stop = state.last_stop.write().unwrap();

            if last_stop.is_some() {
                if last_stop.as_ref().unwrap().has_reached {
                    let last_stop = last_stop.as_mut().unwrap();
                    // should be safe to unwrap here, since if there's a stop, there must be a message
                    let last_message = messages.last().unwrap();
                    let next_stop = get_next_bus_stop(
                        last_stop,
                        LatLng { lat: last_message.lt, lng: last_message.lg },
                    );
                    *last_stop = next_stop;
                } else {
                    // Just an update of its distance and timestamp
                    if let Some(last_stop) = last_stop.as_mut() {
                        last_stop.distance = get_distance_to_bus_stop(
                            last_stop,
                            LatLng { lat: message.lt, lng: message.lg },
                        );
                        last_stop.timestamp = SystemTime::now();
                    }
                }
            }
        },
    }

    message.timestamp = Some(SystemTime::now()); // Add the current timestamp
    let mut messages_copy = messages.clone();
    messages_copy.push(message.clone());

    message.velocity = utils::calculate_velocity_kmph(messages_copy.as_slice());

    drop(messages);
    let mut messages = state.messages.write().unwrap();

    messages.push(message);
    if messages.len() > 100 {
        messages.remove(0); // Keep only the latest 100 positions
    }
    Status::Ok
}
