use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use serde_json::json;

use crate::core::types::ApiResponse;
use crate::core::{responses, AppState};
use crate::features::updates::schemas;

pub async fn delete_app_version_handler(id: i32, state: &State<AppState>) -> ApiResponse {
    let deleted_version = sqlx::query_as!(
        schemas::AppVersion,
        "DELETE FROM app_versions WHERE id = $1 RETURNING *;",
        id,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    if deleted_version.is_none() {
        return Err(status::Custom(
            Status::NotFound,
            responses::error_response("Flag not found".to_string()),
        ));
    }

    Ok(json!(deleted_version.unwrap()))
}
