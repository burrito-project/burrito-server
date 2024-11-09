use rocket::Route;

use crate::core::utils::with_base;

pub mod login;
pub mod profile;

// nested_router!(AuthRouter, [AuthLoginRouter]);

#[derive(utoipa::OpenApi)]
#[openapi(
    nest(
        (path = "/login", api = login::AuthLoginRouter),
        (path = "/profile", api = profile::AuthProfileRouter),
    )
)]
pub struct AuthRouter;

impl crate::routes::ApiRouter for AuthRouter {
    fn routes() -> Vec<rocket::Route> {
        let login_routes = with_base(login::AuthLoginRouter::routes(), "/login");
        let profile_routes = with_base(profile::AuthProfileRouter::routes(), "/profile");

        login_routes.chain(profile_routes).collect::<Vec<Route>>()
    }
}
