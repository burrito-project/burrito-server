use rocket::{http::Status, Response, Route, State};

use crate::core::{responses::RawResponse, types::JsonResult};
use crate::entities::AppState;
use crate::features::bot::{
    self,
    entities::{WhastappMessageType, WhatsappMessage},
    handlers::verify::PubVerificationParams,
};

pub fn routes() -> Vec<Route> {
    routes![whatsapp_new_message, whatsapp_verify]
}

#[post("/", data = "<message>")]
async fn whatsapp_new_message(
    state: &State<AppState>,
    message: JsonResult<'_, WhatsappMessage>,
) -> RawResponse<'static> {
    if message.is_err() {
        // Send ok to avoid retries from WhatsApp
        return Response::build().status(Status::Ok).finalize().into();
    }

    let message = message.unwrap().into_inner();
    let message_type = &message.entry[0].changes[0].value.messages[0].r#type;

    match message_type {
        WhastappMessageType::Text
        | WhastappMessageType::Sticker
        | WhastappMessageType::Audio
        | WhastappMessageType::Video
        | WhastappMessageType::Image => {
            bot::handlers::new_message::whatsapp_new_text_message_handler(state, message).await
        }
        WhastappMessageType::Interactive => {
            bot::handlers::new_interaction::whatsapp_new_interaction_handler(message).await
        }
    }
}

/// Endpoint for WhatsApp webhook verification.
///
/// In the Meta App Dashboard, go to WhatsApp > Configuration > Webhook and use this route
/// as the webhook URL, with `env::AUTH_WHATSAPP_ACCESS_TOKEN` as the authorization token.
#[get("/?<hub>")]
fn whatsapp_verify(hub: PubVerificationParams) -> RawResponse<'static> {
    bot::handlers::verify::whatsapp_verify_handler(hub)
}
