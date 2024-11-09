use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct AppUser {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub is_active: bool,
    #[serde(skip_serializing)]
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub is_staff: bool,
    #[serde(skip_serializing)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AppUser {
    #[allow(dead_code)]
    pub fn is_root(&self) -> bool {
        self.id == 0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct OptAppUser {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub password_hash: Option<String>,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<OptAppUser> for AppUser {
    type Error = ();

    fn try_from(opt: OptAppUser) -> Result<Self, Self::Error> {
        Ok(Self {
            id: opt.id.ok_or(())?,
            username: opt.username.ok_or(())?,
            display_name: opt.display_name.ok_or(())?,
            password_hash: opt.password_hash.ok_or(())?,
            is_active: opt.is_active.ok_or(())?,
            is_staff: opt.is_staff.ok_or(())?,
            last_login: opt.last_login,
            created_at: opt.created_at.ok_or(())?,
            updated_at: opt.updated_at.ok_or(())?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserLoginPayload {
    pub username: String,
    pub password: String,
}

// JWT claims

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    /// Subject (user ID)
    pub sub: i32,
    /// Issued at (UNIX timestamp)
    pub iat: i64,
}

impl JWTClaims {
    pub fn root() -> Self {
        Self {
            sub: 0,
            iat: chrono::Utc::now().timestamp(),
        }
    }

    pub fn new(sub: i32) -> Self {
        Self {
            sub,
            iat: chrono::Utc::now().timestamp(),
        }
    }
}
