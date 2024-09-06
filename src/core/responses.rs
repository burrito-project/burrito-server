use rocket::{
    response::{self, Responder},
    Request,
};
use serde::Serialize;
use serde_json::json;

pub struct RawResponse<'r> {
    response: response::Response<'r>,
}

impl<'r> RawResponse<'r> {
    #[allow(dead_code)]
    pub fn from(response: response::Response<'r>) -> RawResponse<'r> {
        RawResponse { response }
    }
}

impl<'r> Into<RawResponse<'r>> for response::Response<'r> {
    fn into(self) -> RawResponse<'r> {
        RawResponse { response: self }
    }
}

impl<'r> Responder<'r, 'r> for RawResponse<'r> {
    fn respond_to(self, _req: &Request) -> response::Result<'r> {
        Ok(self.response)
    }
}

pub fn error_response(message: impl Into<String> + Serialize) -> serde_json::Value {
    json!({ "message": message })
}
