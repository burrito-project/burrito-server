use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::{ApiResponse, BurritoAPIError};
use crate::core::AppState;
use crate::features::updates::schemas;

pub async fn delete_app_version_handler(
    id: i32,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::AppVersion>> {
    let deleted_version = sqlx::query_as!(
        schemas::AppVersion,
        "DELETE FROM app_versions WHERE id = $1 RETURNING *;",
        id,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    if deleted_version.is_none() {
        return BurritoAPIError::not_found("Flag not found");
    }

    Ok(Json(deleted_version.unwrap()))
}
