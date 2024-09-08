use rocket::Route;

use crate::core::utils::with_base;

mod crash_reports;

pub fn routes() -> Vec<Route> {
    let crash_reports_routes = with_base(crash_reports::routes(), "/crash_reports");

    crash_reports_routes.collect::<Vec<Route>>()
}
