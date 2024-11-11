use rocket::{http::Status, Response};
use utoipa::ToSchema;

use crate::core::responses::RawResponse;

#[derive(FromForm, ToSchema)]
pub struct PubVerificationParams {
    mode: String,
    verify_token: String,
    challenge: String,
}

pub fn whatsapp_verify_handler(hub: PubVerificationParams) -> RawResponse<'static> {
    let bad_request = Response::build()
        .status(Status::BadRequest)
        .finalize()
        .into();

    if hub.mode != "subscribe" {
        return bad_request;
    }
    if hub.verify_token != *crate::env::AUTH_WHATSAPP_ACCESS_TOKEN {
        return bad_request;
    }

    Response::build()
        .status(Status::Ok)
        .sized_body(hub.challenge.len(), std::io::Cursor::new(hub.challenge))
        .header(rocket::http::ContentType::JSON)
        .finalize()
        .into()
}
