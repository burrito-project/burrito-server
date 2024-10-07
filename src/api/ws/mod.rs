use rocket::Route;

use crate::core::utils::with_base;

mod driver;
mod status;

pub fn routes() -> Vec<Route> {
    let driver_routes = with_base(driver::routes(), "/driver");
    let status_routes = with_base(status::routes(), "/status");

    driver_routes.chain(status_routes).collect::<Vec<Route>>()
}
