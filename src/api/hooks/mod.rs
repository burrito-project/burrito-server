use rocket::Route;

use crate::core::utils::with_base;

mod whatsapp;

pub fn routes() -> Vec<Route> {
    let driver_routes = with_base(whatsapp::routes(), "/whatsapp");

    driver_routes.collect::<Vec<Route>>()
}
