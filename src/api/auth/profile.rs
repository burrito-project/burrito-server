use rocket::Route;

use crate::core::types::ApiResponse;
use crate::features::auth;

pub fn routes() -> Vec<Route> {
    routes![get_user_profile]
}

#[get("/")]
pub async fn get_user_profile(app_user: auth::schemas::AppUser) -> ApiResponse {
    auth::handlers::get_user_profile_handler(app_user).await
}
