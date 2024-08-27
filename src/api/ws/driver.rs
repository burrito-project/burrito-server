use crate::{
    auth::ExclusiveAuthDriver,
    bus_stops::{get_bus_stop_for_point, LatLng},
    core::utils,
    entities::{AppState, BurritoPosRecord, BurritoRecordPayload, WsClientMessage},
};
use rocket::{Route, State};
use tokio::sync::broadcast::error::SendError;

pub fn routes() -> Vec<Route> {
    routes![ws_driver_streaming]
}

#[get("/")]
async fn ws_driver_streaming(
    driver: ExclusiveAuthDriver,
    state: &State<AppState>,
    ws: ws::WebSocket,
) -> ws::Channel<'_> {
    use rocket::futures::StreamExt;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let data = match message {
                    Ok(ref msg) => match msg.to_text() {
                        Ok(msg) => msg,
                        Err(_) => break,
                    },
                    Err(_) => break,
                };

                if data.is_empty() {
                    continue;
                }

                let payload: BurritoRecordPayload = match serde_json::from_str(data) {
                    Ok(record) => record,
                    Err(e) => {
                        eprintln!("Error parsing driver's JSON: {}", e);
                        continue;
                    }
                };

                let _ = driver_message_impl(payload, state).await;
            }

            // We must release the lock when the driver disconnects
            state.drivers_locks.lock().await.remove(&driver.bus_name);
            Ok(())
        })
    })
}

pub async fn driver_message_impl(
    payload: BurritoRecordPayload,
    state: &State<AppState>,
) -> Result<usize, SendError<WsClientMessage>> {
    // 🚍 -- Handling the current bus stop

    let bus_stop = get_bus_stop_for_point(payload.lt, payload.lg);

    // About to replace the latest last_stop:
    let mut last_stop = state.last_stop.write().await;

    // If the bus is right on top of a bus stop, we update the last_stop
    if bus_stop.is_some() {
        *last_stop = bus_stop.clone();
    }
    // Otherwise, we can gather the next bus stop information by
    // reading the last_stop
    if bus_stop.is_none() && last_stop.is_some() {
        let last_stop = last_stop.as_mut().unwrap();

        *last_stop = last_stop.for_new_position(LatLng {
            lat: payload.lt,
            lng: payload.lg,
        });
    }

    // 🚍 -- Handling the next record to append

    let mut messages = state.records.write().await;

    messages.push(BurritoPosRecord {
        lt: payload.lt,
        lg: payload.lg,
        sts: payload.sts,
        timestamp: std::time::SystemTime::now(), // Add the current timestamp,
        // pending to calculate
        velocity: 0.0,
    });
    messages.last_mut().unwrap().velocity = utils::calculate_velocity_kmph(&messages);

    if messages.len() > *crate::env::MAX_MEMORY_RECORDS {
        messages.remove(0); // Keep only the latest 1000 positions
    }

    let message_to_send = WsClientMessage {
        last_stop: bus_stop,
        record: messages.last().unwrap().clone(),
    };

    let channel = state.channel.clone();

    channel.send(message_to_send)
}
