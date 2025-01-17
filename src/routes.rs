use rocket::{fs, Build, Rocket};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use crate::{
    api::{
        analytics::AnalyticsRouter, auth::AuthRouter, battery::BatteryRouter, driver::DriverRouter,
        flags::FlagsRouter, health::HealthRouter, hooks::HooksRouter, index::IndexRouter,
        map::MapsRouter, notifications::NotificationsRouter, pending_updates::PendingUpdatesRouter,
        session::SessionRouter, status::StatusRouter, versions::VersionsRouter, ws::WsRouter,
    },
    docs::ApiDocs,
};
pub(crate) use internal::ApiRouter;

pub(crate) fn api_routers() -> Vec<internal::ApiRouterInternal> {
    use internal::mount_router;

    vec![
        mount_router::<WsRouter>("/ws"),
        mount_router::<IndexRouter>("/"),
        mount_router::<MapsRouter>("/map"),
        mount_router::<AuthRouter>("/auth"),
        mount_router::<HooksRouter>("/hooks"),
        mount_router::<FlagsRouter>("/flags"),
        mount_router::<StatusRouter>("/status"),
        mount_router::<DriverRouter>("/driver"),
        mount_router::<HealthRouter>("/health"),
        mount_router::<BatteryRouter>("/battery"),
        mount_router::<SessionRouter>("/session"),
        mount_router::<VersionsRouter>("/versions"),
        mount_router::<AnalyticsRouter>("/analytics"),
        mount_router::<NotificationsRouter>("/notifications"),
        mount_router::<PendingUpdatesRouter>("/pending_updates"),
    ]
}

/// Mounts all the API routers and the API documentation.
pub(crate) fn mount_routers(mut rocket: Rocket<Build>) -> Rocket<Build> {
    for router in api_routers() {
        rocket = rocket.mount(router.base, router.routes);
    }

    rocket
        .mount("/docs", Scalar::with_url("/", ApiDocs::openapi()))
        .mount("/public", fs::FileServer::from(fs::relative!("public")))
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
        "message": "That's a certified 404 classic. Lost? Try /docs",
    })
}
