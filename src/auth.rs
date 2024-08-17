use rocket::{http::Status, request::FromRequest};

pub struct WithAuth();

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WithAuth {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let auth = match request.headers().get("authorization").next() {
            Some(auth) => auth,
            None => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            }
        };

        if auth != crate::env::AUTH_DRIVER_PASSPHRASE.as_str() {
            return rocket::request::Outcome::Forward(Status::Unauthorized);
        }

        rocket::request::Outcome::Success(WithAuth {})
    }
}
