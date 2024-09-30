use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Flag {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub value: bool,
    pub internal: bool,
    pub protected: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct FlagPayload {
    pub value: bool,
}
