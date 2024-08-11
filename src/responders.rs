use rocket::{response::{self, Responder}, Request};

pub struct RawResponse<'r> {
    response: response::Response<'r>,
}

impl<'r> RawResponse<'r> {
    pub fn from(response: response::Response<'r>) -> RawResponse<'r> {
        RawResponse { response }
    }
}

impl<'r> Responder<'r, 'r> for RawResponse<'r> {
    fn respond_to(self, _req: &Request) -> response::Result<'r> {
        Ok(self.response)
    }
}
