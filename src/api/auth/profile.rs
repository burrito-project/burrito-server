use rocket::serde::json::Json;

use crate::core::types::ApiResponse;
use crate::features::auth;
use crate::{docs, router};

router!(AuthProfileRouter, [get_user_profile]);

#[utoipa::path(
    tag = docs::tags::AUTH_TAG,
    description = "Get the user's profile information for displaying it in your shoddy UI.",
    responses(
        (status = 200, body = auth::schemas::AppUser),
        (status = 401),
    ),
    security(
        ("staff_user_auth" = []),
        ("super_user_auth" = []),
    )
)]
#[get("/")]
pub async fn get_user_profile(
    app_user: auth::schemas::AppUser,
) -> ApiResponse<Json<auth::schemas::AppUser>> {
    auth::handlers::get_user_profile_handler(app_user).await
}
