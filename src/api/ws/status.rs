use rocket::{futures::StreamExt, Route, State};

use crate::core::AppState;

pub fn routes() -> Vec<Route> {
    routes![ws_status_streaming]
}

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
