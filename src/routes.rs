use rocket::{Build, Rocket, Route};
use serde_json::json;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_redoc::{Redoc, Servable as RedocServable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use crate::api::{battery::BatteryRouter, driver::DriverRouter};

pub struct ApiRouterInternal {
    base: String,
    routes: Vec<Route>,
    openapi: utoipa::openapi::OpenApi,
}

pub trait ApiRouter: OpenApi {
    fn routes() -> Vec<Route>;
}

fn mount_router<R: ApiRouter + OpenApi>(mount: &'static str) -> ApiRouterInternal {
    ApiRouterInternal {
        base: mount.into(),
        routes: R::routes(),
        openapi: R::openapi(),
    }
}

pub fn routers() -> Vec<ApiRouterInternal> {
    vec![
        mount_router::<BatteryRouter>("/battery"),
        mount_router::<DriverRouter>("/driver"),
    ]
}

pub fn mount_routers(mut rocket: Rocket<Build>) -> Rocket<Build> {
    for router in routers() {
        rocket = rocket.mount(router.base, router.routes);
    }

    rocket
        .mount("/docs", Redoc::with_url("/redoc", ApiDoc::openapi()))
        .mount("/docs", Scalar::with_url("/scalar", ApiDoc::openapi()))
        .register("/", catchers![not_found])
}

pub struct ApiDoc;

impl OpenApi for ApiDoc {
    fn openapi() -> utoipa::openapi::OpenApi {
        let mut open_api = utoipa::openapi::OpenApiBuilder::new()
            .info(
                utoipa::openapi::InfoBuilder::new()
                    .title("Burrito API")
                    .version(env!("CARGO_PKG_VERSION"))
                    .description(Some(env!("CARGO_PKG_DESCRIPTION")))
                    .contact(Some(
                        utoipa::openapi::ContactBuilder::new()
                            .name(Some("@paoloose"))
                            .url(Some("https://paoloose.site"))
                            .email(Some("paolo.flores2@unmsm.edu.pe"))
                            .build(),
                    ))
                    .build(),
            )
            .components(Some(utoipa::openapi::Components::new()))
            .build();

        for router in routers() {
            open_api = open_api.nest(router.base, router.openapi);
        }

        SecurityAddon.modify(&mut open_api);
        open_api
    }
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        )
    }
}

#[catch(404)]
fn not_found() -> serde_json::Value {
    json!({
        "message": "That's a certified 404 classic. Lost? Try /help",
    })
}
