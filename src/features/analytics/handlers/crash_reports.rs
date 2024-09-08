use rocket::{http::Status, response::status, State};
use serde_json::{json, Value};

use crate::{core::responses, entities::AppState, features::analytics::entities};

pub async fn get_crash_reports_handler(
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    let crash_reports = sqlx::query_as!(entities::CrashReport, "SELECT * FROM crash_reports")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => status::Custom(
                Status::BadRequest,
                responses::error_response(db_err.to_string()),
            ),
            e => status::Custom(
                Status::InternalServerError,
                responses::error_response(format!("Failed to get crash reports: {}", e)),
            ),
        })?;

    Ok(json!(crash_reports))
}

pub async fn create_crash_reports_handler(
    issuer: String,
    payload: entities::CrashReportPayload,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
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
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) => status::Custom(
            Status::BadRequest,
            responses::error_response(db_err.to_string()),
        ),
        e => status::Custom(
            Status::InternalServerError,
            responses::error_response(format!("Failed to create crash report: {}", e)),
        ),
    })?;

    Ok(json!(crash_report))
}
