use rocket::response::status;

pub type ApiResponse = Result<serde_json::Value, status::Custom<serde_json::Value>>;
