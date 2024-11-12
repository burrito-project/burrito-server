use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::{routes::api_routers, HOST_URL, SELF_URL};

/// We provide and host our own OpenAPI documentation using
/// [utoipa](https://github.com/juhaku/utoipa), a compile-time OpenAPI generator.
///
/// This class directly consumes the routes defined in crate::routes::api_routers
/// and nests them into a single OpenAPI document.
pub struct ApiDocs;

impl OpenApi for ApiDocs {
    /// See <https://docs.rs/utoipa/latest/utoipa/derive.OpenApi.html>
    fn openapi() -> utoipa::openapi::OpenApi {
        let mut open_api = utoipa::openapi::OpenApiBuilder::new()
            .info(
                utoipa::openapi::InfoBuilder::new()
                    .title("Burrito API")
                    .version(env!("CARGO_PKG_VERSION"))
                    .description(Some(
                        concat!(
                            env!("CARGO_PKG_DESCRIPTION"),
                            "\n![App logo](/public/img/banner.png)"
                        )
                    ))
                    .contact(Some(
                        // Reach out me everywhere 🐢
                        utoipa::openapi::ContactBuilder::new()
                            .name(Some("@paoloose"))
                            .url(Some("https://paoloose.site"))
                            .email(Some("paolo.flores2@unmsm.edu.pe"))
                            .build(),
                    ))
                    .build(),
            )
            .servers(Some(vec![
                utoipa::openapi::ServerBuilder::new()
                    .url(HOST_URL)
                    .description(Some("Production server"))
                    .build(),
                utoipa::openapi::ServerBuilder::new()
                    .url(SELF_URL)
                    .description(Some("Local testing server"))
                    .build(),
            ]))
            .tags(Some(vec![
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::BUS_INFO_TAG)
                    .description(Some(
                        "Endpoints related to real-time bus information. This includes bus status,
                        bus stops, device battery and more.\n\nThese are the only endpoints that
                        simple clients would probably ever need.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::BUS_DRIVER_TAG)
                    .description(Some(
                        "Endpoints related to bus driver communication. This includes bus
                        driver status, location, battery and more. Driver use only."
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::FEATURE_FLAGS_TAG)
                    .description(Some("Endpoints related to Feature flags. Feature flags are meant to be used in both client and server side to enable/disable features without deploying new code."))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::MAP_RENDERING_TAG)
                    .description(Some("Endpoints related to map image rendering. This feature is still in development."))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::AUTH_TAG)
                    .description(Some("Authentication related endpoints, including user auth, driver auth, admin auth and more."))
                    .build(),
            ]))
            .components(Some(utoipa::openapi::Components::new()))
            .build();

        for router in api_routers() {
            open_api = open_api.nest(router.base, router.openapi);
        }

        SecurityAddon.modify(&mut open_api);
        open_api
    }
}

pub mod tags {
    pub const SERVER_TAG: &str = "Server status";
    pub const BUS_INFO_TAG: &str = "Bus information";
    pub const BUS_DRIVER_TAG: &str = "Bus driver communication";
    pub const FEATURE_FLAGS_TAG: &str = "Feature flags";
    pub const APP_VERSIONS_TAG: &str = "App versions";
    pub const NOTIFICATIONS_TAG: &str = "App notifications";
    pub const MAP_RENDERING_TAG: &str = "Map rendering";
    pub const AUTH_TAG: &str = "Authentication";
    pub const WEBHOOKS_TAG: &str = "Webhooks";
    pub const ANALYTICS_TAG: &str = "Analytics";
    pub const MISC_TAG: &str = "Miscellaneous";
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "staff_user_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );

        components.add_security_scheme(
            "app_user_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );

        components.add_security_scheme(
            "super_user_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );

        components.add_security_scheme(
            "driver_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}
