use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

#[derive(Debug, sqlx::Type, Default, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlatformType {
    Android,
    Ios,
    Web,
    #[default]
    All,
}

impl From<String> for PlatformType {
    fn from(value: String) -> Self {
        PlatformType::try_from(value.to_lowercase().as_str()).unwrap_or_default()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppVersion {
    pub id: i32,
    pub semver: String,
    pub platform: PlatformType,
    pub should_notify: bool,
    pub is_mandatory: bool,
    pub banner_url: Option<String>,
    pub release_notes: Option<String>,
    pub release_date: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppVersionPayload {
    pub semver: String,
    #[serde(default)]
    pub platform: PlatformType,
    pub should_notify: bool,
    pub is_mandatory: bool,
    pub banner_url: Option<String>,
    pub release_notes: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub release_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AppVersionPatchPayload {
    pub semver: Option<String>,
    pub platform: Option<PlatformType>,
    pub should_notify: Option<bool>,
    pub is_mandatory: Option<bool>,
    pub banner_url: Option<String>,
    pub release_notes: Option<String>,
    pub release_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, ToSchema)]
pub struct PendingUpdate {
    pub semver: String,
    pub banner_url: String,
    pub is_mandatory: bool,
    pub release_date: String,
    pub release_notes: String,
}

#[derive(Serialize, ToSchema)]
pub struct PendingUpdatesResponse {
    pub must_update: bool,
    pub versions: Vec<PendingUpdate>,
}
