use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    /// See <https://fetch.spec.whatwg.org/#http-responses>
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses for a CORS request and CORS-preflight requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == rocket::http::Method::Options {
            response.set_status(Status::Ok);
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Max-Age", "86400"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, PATCH, PUT, DELETE, OPTIONS",
            ));
        }
    }
}
