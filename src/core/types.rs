use rocket::response::status;
use rocket::serde as rocket_serde;

/// Re-export
pub use rocket::serde::json::Json;

pub type ApiResponse = Result<serde_json::Value, status::Custom<serde_json::Value>>;

// pub type ApiResponse2<T, E> = Result<T, status::Custom<E>>;

pub type JsonResult<'a, T> = Result<rocket_serde::json::Json<T>, rocket_serde::json::Error<'a>>;
