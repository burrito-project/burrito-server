use rocket::{
    response::{self, Responder},
    Request,
};

pub struct RawResponse<'r> {
    response: response::Response<'r>,
}

impl<'r> RawResponse<'r> {
    #[allow(dead_code)]
    pub fn from(response: response::Response<'r>) -> RawResponse<'r> {
        RawResponse { response }
    }
}

impl<'r> From<response::Response<'r>> for RawResponse<'r> {
    fn from(val: response::Response<'r>) -> Self {
        RawResponse { response: val }
    }
}

impl<'r> Responder<'r, 'r> for RawResponse<'r> {
    fn respond_to(self, _req: &Request) -> response::Result<'r> {
        Ok(self.response)
    }
}
