use rocket::State;
use serde_json::json;

use crate::{core::types::ApiResponse, entities::AppState, features::analytics::entities};

pub async fn get_crash_reports_handler(state: &State<AppState>) -> ApiResponse {
    let crash_reports = sqlx::query_as!(entities::CrashReport, "SELECT * FROM crash_reports;")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Ok(json!(crash_reports))
}

pub async fn create_crash_reports_handler(
    issuer: String,
    payload: entities::CrashReportPayload,
    state: &State<AppState>,
) -> ApiResponse {
    let crash_report = sqlx::query_as!(
        entities::CrashReport,
        "INSERT INTO crash_reports
        (issuer, error, stacktrace)
        VALUES ($1, $2, $3)
        RETURNING *;",
        issuer,
        payload.error,
        payload.stacktrace,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(json!(crash_report))
}
