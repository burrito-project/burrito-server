use serde::Serialize;
use serde_json::json;

pub fn error_response(message: impl Into<String> + Serialize) -> serde_json::Value {
    json!({ "message": message })
}
