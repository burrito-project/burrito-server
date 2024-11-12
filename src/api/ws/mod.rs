use rocket::Route;

use crate::core::utils::with_base;

mod driver;
mod status;

#[derive(utoipa::OpenApi)]
#[openapi(
    nest(
        (path = "/driver", api = driver::WsDriverRouter),
        (path = "/status", api = status::WsStatusRouter),
    )
)]
pub struct WsRouter;

impl crate::routes::ApiRouter for WsRouter {
    fn routes() -> Vec<rocket::Route> {
        let driver_routes = with_base(driver::WsDriverRouter::routes(), "/driver");
        let status_routes = with_base(status::WsStatusRouter::routes(), "/status");

        driver_routes.chain(status_routes).collect::<Vec<Route>>()
    }
}
