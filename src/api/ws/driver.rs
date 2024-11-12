use rocket::State;

use crate::core::AppState;
use crate::features::auth::guards::ExclusiveAuthDriver;
use crate::features::bus_driver;
use crate::features::bus_driver::schemas::BurritoRecordPayload;
use crate::{docs, router};

router!(WsDriverRouter, [ws_driver_message_streaming]);

#[utoipa::path(
    tag = docs::tags::BUS_DRIVER_TAG,
    description =
        "WebSocket equivalent of the `/driver` endpoint. Actually, they share the same code,
        and thus the same functionality.
        \nThe messages share the same [BurritoRecordPayload](#model/burritorecordpayload)
        schema and notify the clients in the same way.
        \nThe communication is one-way, from the driver to the server.",
    params(
        (
            "x-bus-id" = String, Header,
            description = "Unique bus driver identifier. Aims to support multiple bus drivers at the same time.",
            example = "burrito-001",
        ),
    ),
    security(("driver_auth" = [])),
    responses(
        (status = 101, description = "Switching protocols"),
        (status = 200),
        (status = 401),
        (status = 429, description = "There is a driver with the same `x-bus-id` already connected"),
    ),
)]
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
