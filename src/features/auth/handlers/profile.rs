use rocket::response::status;
use serde_json::{json, Value};

use crate::features::auth;

pub async fn get_user_profile_handler(
    app_user: auth::schemas::AppUser,
) -> Result<Value, status::Custom<Value>> {
    Ok(json!(app_user))
}
