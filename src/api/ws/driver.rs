use rocket::{Route, State};

use crate::{
    auth::AuthDriver,
    entities::{AppState, BurritoStateRecord, WsClientMessage, WsDriverMessage},
};

pub fn routes() -> Vec<Route> {
    routes![ws_driver_streaming]
}

#[get("/")]
async fn ws_driver_streaming(
    ws: ws::WebSocket,
    state: &State<AppState>,
    driver: AuthDriver,
) -> ws::Channel<'_> {
    use rocket::futures::StreamExt;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                if message.is_err() {
                    break;
                }
                let message = message.unwrap();

                match message.to_text() {
                    Ok(data) => {
                        if data.is_empty() {
                            continue;
                        }

                        let record: WsDriverMessage = match serde_json::from_str(data) {
                            Ok(record) => record,
                            Err(e) => {
                                eprintln!("Error parsing driver's JSON: {}", e);
                                continue;
                            }
                        };

                        let channel = state.channel.clone();
                        let _ = channel.send(WsClientMessage {
                            last_stop: None,
                            record: BurritoStateRecord {
                                lt: record.lt,
                                lg: record.lg,
                                sts: record.sts,
                                timestamp: std::time::SystemTime::now(),
                                velocity: 0.0,
                            },
                        });
                    }
                    Err(e) => {
                        eprintln!("Error parsing driver's message: {}", e);
                        break;
                    }
                }
            }

            // We must release the lock when the driver disconnects
            state.drivers_locks.lock().await.remove(&driver.bus_name);
            Ok(())
        })
    })
}
