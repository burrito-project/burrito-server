use rocket::{futures::StreamExt, State};

use crate::{core::AppState, docs, features::bus_driver, router};

router!(WsStatusRouter, [ws_status_streaming]);

#[utoipa::path(
    tag = docs::tags::BUS_INFO_TAG,
    description =
        "WebSocket endpoint for clients to receive the latest bus status updates.
        The message schema is defined in the body of the `200` response.
        \nFor every bus update, a message is broadcasted to all the clients connected.
        \nAlthough the official client does not use this endpoint, it is considered production ready.",
    responses(
        (status = 101, description = "Switching protocols"),
        (status = 200, description = "", body = bus_driver::schemas::WsClientMessage),
    ),
)]
#[get("/")]
fn ws_status_streaming(ws: ws::WebSocket, state: &State<AppState>) -> ws::Channel<'_> {
    use rocket::futures::SinkExt;

    ws.channel(move |stream| {
        Box::pin(async move {
            // We suscribe to the channel and drop the reference
            let channel = state.channel.clone();
            let mut tx = channel.subscribe();
            drop(channel);

            let mut stream = stream.fuse();

            loop {
                tokio::select! {
                    Ok(driver_message) = tx.recv() => {
                        let driver_message = ws::Message::Text(
                            serde_json::to_string(&driver_message).unwrap(),
                        );

                        match stream.send(driver_message).await {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    },
                    Some(message) = stream.next() => {
                        let message = message?;
                        let message = message.to_text()?;

                        if message.is_empty() {
                            break;
                        }
                    },
                    else => break,
                }
            }

            Ok(())
        })
    })
}
