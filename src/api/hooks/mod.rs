use rocket::Route;

use crate::core::utils::with_base;

mod whatsapp;

#[derive(utoipa::OpenApi)]
#[openapi(
    nest(
        (path = "/whatsapp", api = whatsapp::HooksWhatsappRouter),
    )
)]
pub struct HooksRouter;

impl crate::routes::ApiRouter for HooksRouter {
    fn routes() -> Vec<rocket::Route> {
        let driver_routes = with_base(whatsapp::HooksWhatsappRouter::routes(), "/whatsapp");

        driver_routes.collect::<Vec<Route>>()
    }
}
