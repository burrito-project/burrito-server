use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

#[derive(Debug, sqlx::Type, Default, Deserialize, Serialize, EnumString, Display, ToSchema)]
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AppVersion {
    pub id: i32,
    /// The version code. Should be the same as the one in the pubspec.yaml file.
    #[schema(example = "0.6.9")]
    pub semver: String,
    /// The platform for which this version is intended. Default is All.
    pub platform: PlatformType,
    /// Whether users should be notified about this update. Clients may show a popup to the users
    /// with the new version information.
    pub should_notify: bool,
    /// Whether the new version is mandatory to update or not. Set this to true if the update could
    /// break the app behavior or if it is a security update. The client implementation may
    /// restrict access to the app until the user updates to the new version.
    pub is_mandatory: bool,
    /// If not null, the New Version Popup shows a banner to the users. The banner is a visual
    /// representation of what the new version is about to attract users attention.
    pub banner_url: Option<String>,
    /// If not null, this is a summary of the changes that are introduced in the new version.
    pub release_notes: Option<String>,
    /// The date when this update was released. This does not affect the app behavior, it is just
    /// for information purposes.
    pub release_date: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[allow(unused)]
pub struct AppVersionPayload {
    /// The version code. Should be the same as the one in the pubspec.yaml file.
    pub semver: String,
    /// The platform for which this version is intended. Default is All.
    #[serde(default)]
    pub platform: PlatformType,
    /// Whether users should be notified about this update. Clients may show a popup to the users
    /// with the new version information.
    pub should_notify: bool,
    /// Whether the new version is mandatory to update or not. Set this to true if the update could
    /// break the app behavior or if it is a security update. The client implementation may
    /// restrict access to the app until the user updates to the new version.
    pub is_mandatory: bool,
    /// If not null, the New Version Popup shows a banner to the users. The banner is a visual
    /// representation of what the new version is about to attract users attention.
    pub banner_url: Option<String>,
    /// If not null, this is a summary of the changes that are introduced in the new version.
    pub release_notes: Option<String>,
    /// The date-time notation (as defined by RFC 3339) when this update was released.
    /// This does not affect the app behavior, it is just for information purposes.
    /// Default is now.
    #[serde(default = "chrono::Utc::now")]
    pub release_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AppVersionPatchPayload {
    #[schema(example = "2.6.9")]
    pub semver: Option<String>,
    pub platform: Option<PlatformType>,
    #[schema(example = true)]
    pub should_notify: Option<bool>,
    #[schema(example = true)]
    pub is_mandatory: Option<bool>,
    pub banner_url: Option<String>,
    pub release_notes: Option<String>,
    pub release_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, ToSchema)]
pub struct PendingUpdate {
    #[schema(example = "1.6.9")]
    pub semver: String,
    #[schema(example = "https://picsum.photos/id/866/400")]
    pub banner_url: String,
    #[schema(example = true)]
    pub is_mandatory: bool,
    #[schema(example = "2019-10-12T07:20:50.52Z")]
    pub release_date: String,
    #[schema(
        example = "This is a LONG summary of the changes that are introduced in the new version that can even include breaklines."
    )]
    pub release_notes: String,
}

#[derive(Serialize, ToSchema)]
pub struct PendingUpdatesResponse {
    /// Whether, based on the versions available, the user must update the app to continue using it.
    #[schema(example = true)]
    pub must_update: bool,
    /// List of pending updates. This list is sorted by version number in descending order.
    pub versions: Vec<PendingUpdate>,
}
