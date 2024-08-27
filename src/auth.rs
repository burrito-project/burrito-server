use rocket::{
    http::Status,
    request::{self, FromRequest},
};

use crate::entities::AppState;

pub struct ExclusiveAuthDriver {
    pub bus_name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ExclusiveAuthDriver {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, ()> {
        let auth = match request.headers().get("authorization").next() {
            Some(auth) => auth,
            None => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            }
        };

        if auth != crate::env::AUTH_DRIVER_PASSPHRASE.as_str() {
            return rocket::request::Outcome::Forward(Status::Unauthorized);
        }

        let bus_name = match request.headers().get_one("x-bus-id") {
            Some(bus_name) => bus_name.to_string().to_lowercase(),
            None => {
                return rocket::request::Outcome::Forward(Status::BadRequest);
            }
        };

        let app_state = match request.rocket().state::<AppState>() {
            Some(state) => state,
            None => return rocket::request::Outcome::Forward(Status::InternalServerError),
        };

        let mut locks_map = app_state.drivers_locks.lock().await;

        if locks_map.contains_key(&bus_name) {
            return rocket::request::Outcome::Forward(Status::TooManyRequests);
        }

        locks_map.insert(bus_name.clone(), ());
        rocket::request::Outcome::Success(ExclusiveAuthDriver { bus_name })
    }
}
