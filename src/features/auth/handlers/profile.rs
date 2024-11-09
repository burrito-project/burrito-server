use rocket::serde::json::Json;

use crate::{
    core::types::ApiResponse,
    features::auth::{self, schemas::AppUser},
};

pub async fn get_user_profile_handler(
    app_user: auth::schemas::AppUser,
) -> ApiResponse<Json<AppUser>> {
    Ok(Json(app_user))
}
