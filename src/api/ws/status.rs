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
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(300));

            loop {
                interval.tick().await;
                let messages = state.records.read();
                let last_message = messages.last().cloned();
                drop(messages); // we release the lock ASAFP

                let _ = stream
                    .send(ws::Message::Text(
                        serde_json::to_string(&last_message).unwrap(),
                    ))
                    .await;
            }
        })
    })
}
