use rocket::response::status;
use rocket::serde as rocket_serde;

pub type ApiResponse = Result<serde_json::Value, status::Custom<serde_json::Value>>;

pub type JsonResult<'a, T> = Result<rocket_serde::json::Json<T>, rocket_serde::json::Error<'a>>;
