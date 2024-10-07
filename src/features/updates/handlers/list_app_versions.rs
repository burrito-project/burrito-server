use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::updates::schemas;

pub async fn list_app_versions_handler(state: &State<AppState>) -> ApiResponse {
    let versions = sqlx::query_as!(schemas::AppVersion, "SELECT * FROM app_versions;")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Ok(json!(versions))
}
