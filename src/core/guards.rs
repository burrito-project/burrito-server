use std::net::IpAddr;

use rocket::{http::Status, request::FromRequest};

pub struct ForwardedIp(IpAddr);

impl Into<IpAddr> for ForwardedIp {
    fn into(self) -> IpAddr {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ForwardedIp {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let ip_str = match request.headers().get("x-real-ip").next() {
            Some(auth) => auth,
            None => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            }
        };

        let ip_addr: IpAddr = match ip_str.parse() {
            Ok(addr) => addr,
            Err(_) => {
                return rocket::request::Outcome::Forward(Status::BadGateway);
            }
        };

        rocket::request::Outcome::Success(ForwardedIp { 0: ip_addr })
    }
}
