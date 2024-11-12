use rocket::{
    http::Status,
    request::{self, FromRequest},
};
use serde::Deserialize;

use super::consts::{JWT_DECODING_KEY, JWT_VALIDATION, ROOT_USER};
use crate::core::AppState;

#[derive(Clone, Deserialize)]
/// Request guard for routes that requires staff privilegies
/// If the given authorization token happens to be the root secret, then the
/// root user is returned.
pub struct StaffUser(super::schemas::AppUser);

impl From<StaffUser> for super::schemas::AppUser {
    fn from(val: StaffUser) -> Self {
        val.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StaffUser {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, ()> {
        let user_outcome = request.guard::<super::schemas::AppUser>().await;

        let staff_user = match user_outcome {
            rocket::outcome::Outcome::Success(app_user) => {
                if !app_user.is_staff {
                    return rocket::request::Outcome::Forward(Status::Unauthorized);
                }
                StaffUser(app_user)
            }
            rocket::outcome::Outcome::Error(out) => {
                return rocket::request::Outcome::Error(out);
            }
            rocket::outcome::Outcome::Forward(out) => {
                return rocket::request::Outcome::Forward(out);
            }
        };

        rocket::request::Outcome::Success(staff_user)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for super::schemas::AppUser {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, ()> {
        let authorization = match request.headers().get("authorization").next() {
            Some(auth) => auth,
            None => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            }
        };

        // The ROOT user doesn't actually exists in the database. It's a special user
        // that can do anything
        if authorization == crate::env::ROOT_SECRET.as_str() {
            return rocket::request::Outcome::Success(ROOT_USER.clone());
        }

        // If it's not the root secret, we try to decode it as JWT
        let jwt = authorization;

        let result = jsonwebtoken::decode::<super::schemas::JWTClaims>(
            jwt,
            &JWT_DECODING_KEY,
            &JWT_VALIDATION,
        );

        let claims = match result {
            Ok(claims) => claims.claims,
            Err(_) => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            }
        };

        if claims.sub == 0 {
            return rocket::request::Outcome::Success(ROOT_USER.clone());
        }

        let app_state = match request.rocket().state::<AppState>() {
            Some(state) => state,
            None => return rocket::request::Outcome::Forward(Status::InternalServerError),
        };

        let maybe_user = sqlx::query_as!(
            super::schemas::AppUser,
            "SELECT * FROM users where id = $1 and is_active = true",
            claims.sub,
        )
        .fetch_one(&app_state.pool)
        .await;

        let user = match maybe_user {
            Ok(user) => user,
            Err(_) => {
                return rocket::request::Outcome::Forward(Status::Unauthorized);
            }
        };

        rocket::request::Outcome::Success(user)
    }
}

pub struct RootUser(super::schemas::AppUser);

impl From<RootUser> for super::schemas::AppUser {
    fn from(val: RootUser) -> Self {
        val.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RootUser {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, ()> {
        let user_outcome = request.guard::<super::schemas::AppUser>().await;

        let super_user = match user_outcome {
            rocket::outcome::Outcome::Success(app_user) => {
                if !app_user.is_root() {
                    return rocket::request::Outcome::Forward(Status::Unauthorized);
                }
                RootUser(app_user)
            }
            rocket::outcome::Outcome::Error(out) => {
                return rocket::request::Outcome::Error(out);
            }
            rocket::outcome::Outcome::Forward(out) => {
                return rocket::request::Outcome::Forward(out);
            }
        };

        rocket::request::Outcome::Success(super_user)
    }
}

pub struct ExclusiveAuthDriver {
    pub bus_name: String,
}

pub struct AuthDriver {
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthDriver {
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

        rocket::request::Outcome::Success(AuthDriver { bus_name })
    }
}
