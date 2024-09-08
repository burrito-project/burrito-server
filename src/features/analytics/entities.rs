use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CrashReport {
    pub id: i32,
    pub issuer: String,
    pub error: String,
    pub stacktrace: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CrashReportPayload {
    pub error: String,
    pub stacktrace: String,
}
