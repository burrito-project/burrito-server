use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;

#[derive(Debug, sqlx::Type, Default, Deserialize, Serialize, EnumString, ToSchema, Display)]
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Notification {
    pub id: i32,
    pub is_active: bool,
    #[schema(example = "Título de ejemplo")]
    pub ad_title: Option<String>,
    #[schema(example = "Lorem burrosum dolor sit amet, consectetur adipiscing elit.")]
    pub ad_content: Option<String>,
    pub ad_type: AdType,
    pub priority: i32,
    #[schema(example = "https://picsum.photos/id/866/400")]
    pub image_url: Option<String>,
    pub target_url: Option<String>,
    pub begin_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NotificationPayload {
    /// Whether the notification is active. Inactive notifications are not accessible by users.
    pub is_active: bool,
    /// Title of the notification. Only required for `post` and `popup` notifications.
    #[schema(example = "El burrito está en mantenimiento!")]
    pub ad_title: Option<String>,
    /// Content of the notification. Optional. Only required when the notification type is `post`
    /// or `popup`.
    #[schema(example = "Lorem burrosum dolor sit amet, consectetur adipiscing elit.")]
    pub ad_content: Option<String>,
    /// Type of the notification. It can be a `banner` (default), `post` or `popup`.
    ///
    /// Banners are meant to be displayed in a carousel at the bottom of the app.
    ///
    /// Posts are feed-like notifications that are displayed in a list, or along with other banners.
    ///
    /// Popups are meant to be displayed as a modal dialog. Use it for important notifications
    /// or announcements. If you want to announce that a new version was released, use the
    /// `POST /versions` endpoint instead.
    pub ad_type: AdType,
    /// Priority of the notification. 0 is the highest priority.
    pub priority: i32,
    /// Base64 image to display in the notification. The data must start with `data:image/`.
    /// If the image is PNG, the backend will try to compress its size. It's recommended to
    /// compress the image before sending it to the backend. Optional.
    pub image_base64: Option<String>,
    /// URL to redirect the user when the notification is clicked. Optional.
    pub target_url: Option<String>,
    /// Date and time when the notification should start being displayed. If null, the
    /// notification will be displayed immediately.
    pub begin_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Date and time when the notification should stop being displayed.
    /// If null, the notification will be displayed indefinitely.
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}
