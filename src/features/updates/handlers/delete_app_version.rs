use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::updates::schemas;

pub async fn delete_app_version_handler(id: i32, state: &State<AppState>) -> ApiResponse {
    let deleted_version = sqlx::query_as!(
        schemas::AppVersion,
        "DELETE FROM app_versions WHERE id = $1 RETURNING *;",
        id,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(json!(deleted_version))
}
