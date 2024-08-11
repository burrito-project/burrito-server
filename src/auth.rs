use rocket::{http::Status, request::FromRequest};

pub struct WithAuth();

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WithAuth {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let auth = match request.headers().get("authorization").next() {
            Some(auth) => auth,
            None => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            },
        };

        let pass = std::env::var("AUTH_DRIVER_PASSPHRASE").unwrap_or_default();

        if auth != pass {
            return rocket::request::Outcome::Error((Status::Unauthorized, ()));
        }

        rocket::request::Outcome::Success(WithAuth{})
    }
}