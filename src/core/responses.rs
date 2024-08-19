use serde_json::json;

pub fn error_response(message: &'static str) -> serde_json::Value {
    return json!({ "message": message });
}
