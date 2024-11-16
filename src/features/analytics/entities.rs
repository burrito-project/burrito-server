use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct CrashReport {
    pub id: i32,
    #[schema(example = "burrito-driver-001")]
    pub issuer: String,
    #[schema(example = "Null assertion")]
    pub error: String,
    #[schema(example = "Stack trace: #0 ...")]
    pub stacktrace: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CrashReportPayload {
    /// Full error string and additional context that the client may want to provide.
    #[schema(example = "Null assertion")]
    pub error: String,
    /// Full stacktrace of the error, exactly as provided by the client programming language.
    /// If you want to add additional information to the error message, use the `error` field.
    #[schema(example = "Stack trace: #0 ...")]
    pub stacktrace: String,
}
