use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::routes::api_routers;

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
                    .description(Some(concat!(
                        env!("CARGO_PKG_DESCRIPTION"),
                        "\n![App logo](/public/img/banner.png)"
                    )))
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
                    .url(crate::env::HOST_URL.clone())
                    .description(Some("Production server"))
                    .build(),
                utoipa::openapi::ServerBuilder::new()
                    .url(crate::env::SELF_URL.clone())
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
                        driver status, location, battery and more. Driver use only.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::FEATURE_FLAGS_TAG)
                    .description(Some(
                        "Endpoints related to Feature flags. Feature flags are meant to be used
                        in both client and server side to enable/disable features without deploying
                        new code.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::MAP_RENDERING_TAG)
                    .description(Some(
                        "Endpoints related to map image rendering.\n\nThanks to these endpoints you
                        don't even need to build a client to display the bus location on a map.
                        This feature is **still in development**.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::AUTH_TAG)
                    .description(Some(
                        "Authentication related endpoints, including admin auth, staff auth and
                        bus driver auth. Currently, the app doesn't support (nor need) user
                        accounts.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::ANALYTICS_TAG)
                    .description(Some(
                        "Endpoints related to app analytics. Currently, this limited to
                        usage sesions and crash reports.
                        \nTo understand how we use this data, refer to our
                        [privacy policy](https://github.com/burrito-project/public/blob/main/PRIVACY_POLICY.md).",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::SERVER_TAG)
                    .description(Some(
                        "Endpoints related to server status and health. This includes uptime,
                        server version and more.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::WEBHOOKS_TAG)
                    .description(Some(
                        "Webhook endpoints used to notify the server of certain events.
                        Currently we have only registered a bunch of WhatsApp message webhooks
                        events for bot testing.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::MISC_TAG)
                    .description(Some(
                        "Miscellaneous endpoints that don't fit in any other category.
                        This includes app versioning and other stuff.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::NOTIFICATIONS_TAG)
                    .description(Some(
                        "Endpoints related to in-app notifications. No push notifications are
                        supported at the moment.",
                    ))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::APP_VERSIONS_TAG)
                    .description(Some(
                        "Endpoints related to app versioning, from versions history, release
                        notes, checking for updates, or even forcing the users to update their
                        apps.",
                    ))
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
    pub const BUS_INFO_TAG: &str = "Bus information";
    pub const BUS_DRIVER_TAG: &str = "Bus driver communication";
    pub const FEATURE_FLAGS_TAG: &str = "Feature flags";
    pub const MAP_RENDERING_TAG: &str = "Map rendering";
    pub const AUTH_TAG: &str = "Authentication";
    pub const ANALYTICS_TAG: &str = "Analytics";
    pub const SERVER_TAG: &str = "Server status";
    pub const WEBHOOKS_TAG: &str = "Webhooks";
    pub const MISC_TAG: &str = "Miscellaneous";
    pub const NOTIFICATIONS_TAG: &str = "App notifications";
    pub const APP_VERSIONS_TAG: &str = "App versions";
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
            "super_user_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );

        components.add_security_scheme(
            "driver_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}
