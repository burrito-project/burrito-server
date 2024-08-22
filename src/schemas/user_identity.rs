use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;

#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct UserIdentity {
    pub id: i32,
    pub fingerprint: String,
    // ip address type ("ipnetwork" feature)
    pub last_ip: IpNetwork,
    pub last_version: String,
    pub platform: Option<String>,
    pub session_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UserIdentityPayload {
    pub fingerprint: String,
    pub last_version: String,
    pub platform: String,
}
