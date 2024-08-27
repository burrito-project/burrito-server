use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::{Route, State};
use std::time;

use crate::entities::{AppState, BurritoPosRecord, BusServiceState};

pub fn routes() -> Vec<Route> {
    routes![get_status,]
}

const DEFAULT_COUNT: usize = 100;

#[get("/?<count>")]
async fn get_status(count: Option<usize>, state: &State<AppState>) -> Result<Value, Status> {
    let count = count.unwrap_or(DEFAULT_COUNT);
    let messages = state.records.read().await;
    let last_stop = state.last_stop.read().await;

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
                let off_message = BurritoPosRecord {
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

                *state.last_stop.write().await = None;

                return Ok(json!({
                    "positions": messages_cpy.iter().rev().take(n).cloned().collect::<Vec<BurritoPosRecord>>(),
                    "last_stop": null,
                }));
            }

            Ok(json!({
                "positions": messages.iter().rev().take(n).cloned().collect::<Vec<BurritoPosRecord>>(),
                "last_stop": last_stop.clone(),
            }))
        }
        None => Ok(json!({
            "positions": vec![BurritoPosRecord {
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
