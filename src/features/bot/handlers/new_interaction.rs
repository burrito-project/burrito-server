use rocket::{http::Status, Response};
use serde_json::json;

use crate::{
    core::responses::RawResponse,
    features::bot::entities::{WhastappMessageType, WhatsappMessage},
};

pub async fn whatsapp_new_interaction_handler(message: WhatsappMessage) -> RawResponse<'static> {
    println!("{:#?}", &message);

    let number_id = &message.entry[0].changes[0].value.metadata.phone_number_id;
    let message = &message.entry[0].changes[0].value.messages[0];

    debug_assert!(matches!(message.r#type, WhastappMessageType::Interactive));

    let body = json!({
        "messaging_product": "whatsapp",
        "recipient_type": "individual",
        "to": message.from,
        "type": "image",
        "image": {
            "link": format!("{}/map", *crate::env::HOST_URL),
        },
    });
    let client = reqwest::Client::new();

    let reply_result = client
        .post(format!(
            "https://graph.facebook.com/v20.0/{}/messages",
            number_id
        ))
        .header("content-type", "application/json")
        .header(
            "authorization",
            format!("Bearer {}", crate::env::AUTH_WHATSAPP_ACCESS_TOKEN.as_str()),
        )
        .body(body.to_string())
        .send()
        .await;

    match reply_result {
        Ok(w) => {
            println!("{:#?}", w);
        }
        Err(e) => {
            eprintln!("{:#?}", e);
        }
    }
    return Response::build().status(Status::Ok).finalize().into();
}
