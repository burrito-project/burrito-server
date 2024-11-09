use crate::core::types::ApiResponse;
use crate::features::auth;
use crate::{docs, router};

router!(AuthProfileRouter, [get_user_profile]);

#[utoipa::path(
    tag = docs::tags::AUTH_TAG,
)]
#[get("/")]
pub async fn get_user_profile(app_user: auth::schemas::AppUser) -> ApiResponse {
    auth::handlers::get_user_profile_handler(app_user).await
}
