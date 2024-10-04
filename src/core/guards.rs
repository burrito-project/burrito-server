use std::net::IpAddr;

use rocket::{http::Status, request::FromRequest};

pub struct ForwardedIp(IpAddr);

impl From<ForwardedIp> for IpAddr {
    fn from(value: ForwardedIp) -> Self {
        value.0
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

        rocket::request::Outcome::Success(ForwardedIp(ip_addr))
    }
}

pub struct IsMobileChecker(bool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsMobileChecker {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let user_agent = match request.headers().get("user-agent").next() {
            Some(auth) => auth.to_lowercase(),
            None => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            }
        };
        let is_mobile = user_agent.contains("dart:io");
        rocket::request::Outcome::Success(IsMobileChecker(is_mobile))
    }
}

impl IsMobileChecker {
    pub fn ask(&self) -> bool {
        self.0
    }
}
