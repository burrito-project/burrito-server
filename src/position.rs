use std::time::SystemTime;
use rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::bus_stops::{get_bus_stop_for_point, get_distance_to_bus_stop, get_next_bus_stop, LatLng};
use crate::BurritoState;
use crate::Message;
use crate::velocity::calculate_velocity_kmph;

pub fn routes() -> Vec<Route> {
    routes![get_position, give_position, handle_options_request]
}

#[get("/get-position/<count>")]
fn get_position(count: usize, state: &State<BurritoState>) -> Result<Value, Status> {
    let messages = state.messages.lock().unwrap();
    let last_stop = state.last_stop.lock().unwrap();

    let n = std::cmp::min(count, messages.len());

    match messages.last() {
        Some(last) => {
            /*
            status {
                0: en ruta
                1: fuera de servicio
                2: en descanso
                3: accidente
                4: apagado
            }
            */
            let is_off = last.sts == 1 || last.sts == 2 || last.sts == 3;

            // If the burrito didn't report itself as 1,2 or 3 and it hasn't reported in the last 60 seconds,
            // then we consider it as off
            if !is_off && last.timestamp.unwrap().elapsed().unwrap() > std::time::Duration::from_secs(60) {
                // We create an 'off' message on the fly
                let off_message = Message {
                    lt: 0.0,
                    lg: 0.0,
                    sts: 4, // 4 means OFF
                    timestamp: last.timestamp,
                    velocity: 0.0,
                };
                let mut messages_cpy = messages.clone();
                messages_cpy.push(off_message);
                return Ok(json!({
                    "positions": messages_cpy.iter().rev().take(n).cloned().collect::<Vec<Message>>(),
                    "last_stop": null,
                }));
            }
        },
        None => {
            return Ok(json!({
                "positions": vec![Message {
                    lt: 0.0,
                    lg: 0.0,
                    sts: 4, // 4 means OFF
                    timestamp: Some(SystemTime::now()),
                    velocity: 0.0,
                }],
                "last_stop": last_stop.clone(),
            }));
        },
    }

    let recent_messages: Vec<Message> = messages.iter().rev().take(n).cloned().collect();

    Ok(json!({
        "positions": recent_messages,
        "last_stop": last_stop.clone(),
    }))
}

#[post("/give-position", format = "json", data = "<message_json>")]
fn give_position(message_json: Json<Message>, state: &State<BurritoState>) -> Status {
    let mut messages = state.messages.lock().unwrap();
    let mut message = message_json.into_inner();

    match get_bus_stop_for_point(message.lt, message.lg) {
        Some(this_stop) => {
            let mut last_stop = state.last_stop.lock().unwrap();
            // If there's already last_stop we update it
            *last_stop = Some(this_stop);
            // if let Some(last_stop) = last_stop.as_mut() {
            //     if last_stop.number != this_stop.number {
            //         *last_stop = this_stop;
            //     }
            // } else {
            // }
        },
        None => {
            // If the burrito is not in a bus stop and we have a last_stop (has_reached=true),
            // we interpret as it has left that bus stop, so we choose the next one as has_reached=false
            let mut last_stop = state.last_stop.lock().unwrap();
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

    message.velocity = calculate_velocity_kmph(messages_copy.as_slice());

    messages.push(message);
    if messages.len() > 100 {
        messages.remove(0); // Keep only the latest 100 positions
    }
    Status::Ok
}

// NO BORRAR POR FAVOR, SI NO, NO FUNCIONA (CORS'S THING)
#[options("/give-position")]
fn handle_options_request() -> Status {
    Status::NoContent
}
