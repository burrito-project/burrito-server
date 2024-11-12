use rocket::{http::Status, Response, State};

use crate::core::AppState;
use crate::core::{responses::RawResponse, types::JsonResult};
use crate::features::bot::{
    self,
    entities::{WhastappMessageType, WhatsappMessage},
    handlers::verify::PubVerificationParams,
};
use crate::{docs, router};

router!(HooksWhatsappRouter, [whatsapp_new_message, whatsapp_verify]);

#[utoipa::path(
    tag = docs::tags::WEBHOOKS_TAG,
    description = "The callback URL registered in the WhatsApp hooks configuration.
        This is where the incoming messages are received and processed by the bot.
        Internal use only.",
    request_body(content = WhatsappMessage),
)]
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

#[utoipa::path(
    tag = docs::tags::WEBHOOKS_TAG,
    description = "Secured endpoint for WhatsApp webhook verification. Internal use only.")]
#[get("/?<hub>")]
fn whatsapp_verify(hub: PubVerificationParams) -> RawResponse<'static> {
    // In the Meta App Dashboard, go to WhatsApp > Configuration > Webhook and use this route
    // as the webhook URL, with `env::AUTH_WHATSAPP_ACCESS_TOKEN` as the authorization token.

    // An example of how the verification request looks like:
    // GET /hooks/whatsappa?hub.mode=subscribe&hub.challenge=653182446&hub.verify_token=<AUTH_WHATSAPP_ACCESS_TOKEN>:

    bot::handlers::verify::whatsapp_verify_handler(hub)
}
