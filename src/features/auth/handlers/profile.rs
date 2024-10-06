use serde_json::json;

use crate::{core::types::ApiResponse, features::auth};

pub async fn get_user_profile_handler(app_user: auth::schemas::AppUser) -> ApiResponse {
    Ok(json!(app_user))
}
