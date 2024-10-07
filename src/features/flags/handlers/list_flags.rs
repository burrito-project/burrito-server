use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::flags;

pub async fn list_flags_handler(state: &State<AppState>) -> ApiResponse {
    let flags = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE internal = false ORDER BY name ASC;",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Ok(json!(flags))
}
