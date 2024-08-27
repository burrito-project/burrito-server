use rocket::{Route, State};

use crate::entities::AppState;

pub fn routes() -> Vec<Route> {
    routes![ws_status_streaming]
}

#[get("/")]
fn ws_status_streaming(ws: ws::WebSocket, state: &State<AppState>) -> ws::Channel<'_> {
    use rocket::futures::SinkExt;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let channel = state.channel.clone();
            let mut tx = channel.subscribe();
            drop(channel);

            loop {
                let driver_message = tx
                    .recv()
                    .await
                    .map_err(|_| ws::result::Error::AttackAttempt)?;

                let _ = stream
                    .send(ws::Message::Text(
                        serde_json::to_string(&driver_message).unwrap(),
                    ))
                    .await;
            }
        })
    })
}
