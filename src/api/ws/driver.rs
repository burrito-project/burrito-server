use rocket::{Route, State};

use crate::core::AppState;
use crate::features::auth::guards::ExclusiveAuthDriver;
use crate::features::bus_driver;
use crate::features::bus_driver::schemas::BurritoRecordPayload;

pub fn routes() -> Vec<Route> {
    routes![ws_driver_message_streaming]
}

#[get("/")]
async fn ws_driver_message_streaming(
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

                let _ = bus_driver::handlers::driver_message_handler(payload, state).await;
            }

            // We must release the lock when the driver disconnects
            state.drivers_locks.lock().await.remove(&driver.bus_name);
            Ok(())
        })
    })
}
