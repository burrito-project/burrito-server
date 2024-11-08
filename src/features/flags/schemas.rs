use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Flag {
    pub id: i32,
    /// Flag unique identifier
    #[schema(example = "ads_random_order")]
    pub name: String,
    /// Flag displayable name
    #[schema(example = "Anuncios en orden aleatorio")]
    pub display_name: String,
    /// Flag boolean value
    pub value: bool,
    /// Whether the flag is meant for internal use and can't be accessed via API
    pub internal: bool,
    /// Whether the flag is protected and only accessible by admins
    pub protected: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FlagPayload {
    pub value: bool,
}
