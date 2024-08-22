use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, sqlx::Type, Default, Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AdType {
    #[default]
    Banner,
    Post,
    Popup,
}

impl From<String> for AdType {
    fn from(value: String) -> Self {
        AdType::try_from(value.to_lowercase().as_str()).unwrap_or_default()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Advertisement {
    pub id: i32,
    pub is_active: bool,
    pub ad_title: Option<String>,
    pub ad_type: AdType,
    pub priority: i32,
    pub image_url: Option<String>,
    pub target_url: Option<String>,
    pub begin_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ad_content: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AdvertisementPayload {
    pub is_active: bool,
    pub ad_title: Option<String>,
    pub ad_type: AdType,
    pub priority: i32,
    pub image_url: Option<String>,
    pub target_url: Option<String>,
    pub begin_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ad_content: Option<String>,
}
