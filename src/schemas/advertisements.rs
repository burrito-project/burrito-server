use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "platform_t", rename_all = "lowercase")]
pub enum AdType {
    Banner,
    Post,
    Popup,
}

impl TryFrom<&str> for AdType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "banner" => Ok(AdType::Banner),
            "post" => Ok(AdType::Post),
            "popup" => Ok(AdType::Popup),
            _ => Err(format!("Invalid advertisement type: {}", value)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Advertisements {
    pub id: i32,
    pub is_active: bool,
    pub ad_title: Option<String>,
    pub ad_type: AdType,
    pub priority: i32,
    pub image_url: Option<String>,
    pub target_url: Option<String>,
    pub begin_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ad_content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AdvertisementsPayload {
    #[serde(default)]
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
