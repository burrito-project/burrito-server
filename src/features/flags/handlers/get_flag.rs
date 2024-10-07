use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::{responses, AppState};
use crate::features::flags;

pub async fn get_flag_handler(flag: &str, state: &State<AppState>) -> ApiResponse {
    let flag = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE name = $1 AND internal = false LIMIT 1;",
        flag,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    if flag.is_none() {
        return Err(status::Custom(
            Status::NotFound,
            responses::error_response("Flag not found".to_string()),
        ));
    }

    Ok(json!(flag.unwrap().value))
}
