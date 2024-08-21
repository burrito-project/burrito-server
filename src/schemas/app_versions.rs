use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "platform_t", rename_all = "lowercase")]
pub enum PlatformType {
    Android,
    IOS,
    Web,
    All,
}

impl Default for PlatformType {
    fn default() -> Self {
        PlatformType::All
    }
}

impl TryFrom<&str> for PlatformType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "android" => Ok(PlatformType::Android),
            "ios" => Ok(PlatformType::IOS),
            "web" => Ok(PlatformType::Web),
            "all" | "any" => Ok(PlatformType::All),
            _ => Err(format!("Invalid platform: {}", value)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppVersion {
    pub id: i32,
    pub semver: String,
    pub platform: PlatformType,
    pub is_mandatory: bool,
    pub banner_url: Option<String>,
    pub release_notes: Option<String>,
    pub release_date: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AppVersionPayload {
    pub semver: String,
    #[serde(default)]
    pub platform: PlatformType,
    pub is_mandatory: bool,
    pub banner_url: Option<String>,
    pub release_notes: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub release_date: chrono::DateTime<chrono::Utc>,
}
