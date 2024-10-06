use rocket::Route;

use crate::core::utils::with_base;

pub mod login;
pub mod profile;

pub fn routes() -> Vec<Route> {
    let login_routes = with_base(login::routes(), "/login");
    let profile_routes = with_base(profile::routes(), "/profile");

    login_routes.chain(profile_routes).collect::<Vec<Route>>()
}
