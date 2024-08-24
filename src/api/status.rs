use rocket::http::Status;
use rocket::response;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::{Route, State};
use std::time;

use crate::auth::WithAuth;
use crate::bus_stops::{
    get_bus_stop_for_point, get_distance_to_bus_stop, get_next_bus_stop, LatLng,
};
use crate::core::responses::RawResponse;
use crate::core::utils;
use crate::entities::{AppState, BurritoRecordPayload, BurritoStateRecord, BusServiceState};

pub fn routes() -> Vec<Route> {
    routes![get_status, post_status, post_status_unauthorized]
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
            let is_off = matches!(
                last.sts,
                BusServiceState::OutOfService
                    | BusServiceState::Resting
                    | BusServiceState::Accident
            );

            // If the burrito didn't report itself as 1, 2 or 3 and it hasn't reported in the last 60 seconds,
            // then we consider it as off
            if !is_off && last.timestamp.elapsed().unwrap() > std::time::Duration::from_secs(60) {
                // We create an 'off' message on the fly
                let off_message = BurritoStateRecord {
                    lt: 0.0,
                    lg: 0.0,
                    sts: BusServiceState::inherit_from_inactive(last.sts),
                    timestamp: last.timestamp,
                    velocity: 0.0,
                };

                let mut messages_cpy = messages.clone();
                messages_cpy.push(off_message);

                drop(messages);
                drop(last_stop);

                *state.last_stop.write().unwrap() = None;

                return Ok(json!({
                    "positions": messages_cpy.iter().rev().take(n).cloned().collect::<Vec<BurritoStateRecord>>(),
                    "last_stop": null,
                }));
            }

            Ok(json!({
                "positions": messages.iter().rev().take(n).cloned().collect::<Vec<BurritoStateRecord>>(),
                "last_stop": last_stop.clone(),
            }))
        }
        None => Ok(json!({
            "positions": vec![BurritoStateRecord {
                lt: 0.0,
                lg: 0.0,
                sts: BusServiceState::Off,
                timestamp: time::SystemTime::now(),
                velocity: 0.0,
            }],
            "last_stop": last_stop.clone(),
        })),
    }
}

#[post("/", format = "json", data = "<message_json>")]
fn post_status(
    message_json: Json<BurritoRecordPayload>,
    state: &State<AppState>,
    _z: WithAuth,
) -> Status {
    let mut messages = state.messages.write().unwrap();
    let payload = message_json.into_inner();

    match get_bus_stop_for_point(payload.lt, payload.lg) {
        Some(this_stop) => {
            let mut last_stop = state.last_stop.write().unwrap();
            // If there's already last_stop we update it
            *last_stop = Some(this_stop);
        }
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
                        LatLng {
                            lat: last_message.lt,
                            lng: last_message.lg,
                        },
                    );
                    *last_stop = next_stop;
                } else {
                    // Just an update of its distance and timestamp
                    if let Some(last_stop) = last_stop.as_mut() {
                        last_stop.distance = get_distance_to_bus_stop(
                            last_stop,
                            LatLng {
                                lat: payload.lt,
                                lng: payload.lg,
                            },
                        );
                        last_stop.timestamp = time::SystemTime::now();
                    }
                }
            }
        }
    }

    messages.push(BurritoStateRecord {
        lt: payload.lt,
        lg: payload.lg,
        sts: payload.sts,
        timestamp: time::SystemTime::now(), // Add the current timestamp,
        velocity: 0.0,                      // pending to calculate
    });

    messages.last_mut().unwrap().velocity = utils::calculate_velocity_kmph(&messages);

    if messages.len() > *crate::env::MAX_MEMORY_RECORDS {
        messages.remove(0); // Keep only the latest 1000 positions
    }
    Status::Ok
}

#[post("/", rank = 2)]
fn post_status_unauthorized<'r>() -> RawResponse<'r> {
    let mut response = response::Response::new();

    response.set_status(Status::Unauthorized);
    response.set_raw_header("x-message", "Cálmate. Tú no eres así.");

    RawResponse::from(response)
}
