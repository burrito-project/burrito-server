use rocket::{Build, Rocket};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use crate::{
    api::{battery::BatteryRouter, driver::DriverRouter, flags::FlagsRouter, map::MapsRouter},
    docs::ApiDocs,
};
pub(crate) use internal::ApiRouter;

pub(crate) fn api_routers() -> Vec<internal::ApiRouterInternal> {
    use internal::mount_router;

    vec![
        mount_router::<BatteryRouter>("/battery"),
        mount_router::<DriverRouter>("/driver"),
        mount_router::<FlagsRouter>("/flags"),
        mount_router::<MapsRouter>("/maps"),
    ]
}

pub(crate) fn mount_routers(mut rocket: Rocket<Build>) -> Rocket<Build> {
    for router in api_routers() {
        rocket = rocket.mount(router.base, router.routes);
    }

    rocket
        .mount("/docs", Scalar::with_url("/scalar", ApiDocs::openapi()))
        .register("/", catchers![not_found])
}

mod internal {
    pub trait ApiRouter: utoipa::OpenApi {
        fn routes() -> Vec<rocket::Route>;
    }

    pub struct ApiRouterInternal {
        pub base: String,
        pub routes: Vec<rocket::Route>,
        pub openapi: utoipa::openapi::OpenApi,
    }

    pub fn mount_router<R: ApiRouter + utoipa::OpenApi>(mount: &'static str) -> ApiRouterInternal {
        ApiRouterInternal {
            base: mount.into(),
            routes: R::routes(),
            openapi: R::openapi(),
        }
    }
}

#[catch(404)]
fn not_found() -> serde_json::Value {
    json!({
        "message": "That's a certified 404 classic. Lost? Try /help",
    })
}
