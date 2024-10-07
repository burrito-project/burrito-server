use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::flags;

pub async fn get_flag_handler(flag: &str, state: &State<AppState>) -> ApiResponse {
    let flag = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE name = $1 AND internal = false LIMIT 1;",
        flag,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(json!(flag.value))
}
