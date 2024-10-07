use rocket::State;
use tokio::sync::broadcast::error::SendError;

use crate::core::AppState;
use crate::features::bus_driver::schemas::{
    BurritoPosRecord, BurritoRecordPayload, WsClientMessage,
};
use crate::features::bus_driver::utils::calculate_velocity_kmph;
use crate::features::bus_stops::schemas::LatLng;
use crate::features::bus_stops::utils::{get_bus_stop_for_point, OptionalBuStopInfo};

pub async fn driver_message_handler(
    payload: BurritoRecordPayload,
    state: &State<AppState>,
) -> Result<usize, SendError<WsClientMessage>> {
    // 🚍 -- Handling the current bus stop

    // About to replace the latest last_stop:
    let mut last_stop = state.last_stop.write().await;

    // We check if the bus is currently in a bus stop
    let mut bus_stop = get_bus_stop_for_point(payload.lt, payload.lg);

    // Otherwise, we can gather the next bus stop information by
    // reading the last_stop
    if bus_stop.is_none() && last_stop.is_some() {
        bus_stop = last_stop.for_new_position(LatLng {
            lat: payload.lt,
            lng: payload.lg,
        });
    }

    // This may be none and that's ok. At the start of the route, there's no
    // previous bus stop
    *last_stop = bus_stop.clone();
    drop(last_stop);

    // 🚍 -- Handling the next record to append

    let mut messages = state.records.write().await;

    messages.push(BurritoPosRecord {
        lt: payload.lt,
        lg: payload.lg,
        bat: payload.bat,
        sts: payload.sts,
        timestamp: std::time::SystemTime::now(), // Add the current timestamp,
        // pending to calculate
        velocity: 0.0,
    });
    messages.last_mut().unwrap().velocity = calculate_velocity_kmph(&messages);

    if messages.len() > *crate::env::MAX_MEMORY_RECORDS {
        messages.remove(0); // Keep only the latest 1000 positions
    }

    // 🚍 -- Sending the message to the clients

    let message_to_send = WsClientMessage {
        last_stop: bus_stop,
        record: messages.last().unwrap().clone(),
    };
    drop(messages);

    let channel = state.channel.clone();
    channel.send(message_to_send)
}
