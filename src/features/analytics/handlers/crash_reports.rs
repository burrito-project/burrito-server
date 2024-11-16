use rocket::State;

use crate::core::AppState;
use crate::features::analytics::entities;

pub async fn get_crash_reports_handler(state: &State<AppState>) -> Vec<entities::CrashReport> {
    sqlx::query_as!(entities::CrashReport, "SELECT * FROM crash_reports;")
        .fetch_all(&state.pool)
        .await
        .unwrap()
}

pub async fn create_crash_reports_handler(
    issuer: String,
    payload: entities::CrashReportPayload,
    state: &State<AppState>,
) -> entities::CrashReport {
    sqlx::query_as!(
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
    .unwrap()
}
