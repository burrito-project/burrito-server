use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::{routes::api_routers, HOST_URL, SELF_URL};

pub mod tags {
    pub const BUS_INFO_TAG: &'static str = "Bus information";
    pub const BUS_DRIVER_TAG: &'static str = "Bus driver";
    pub const FEATURE_FLAGS_TAG: &'static str = "Feature flags";
    pub const MAP_RENDERING_TAG: &'static str = "Map rendering";
}

pub struct ApiDocs;

impl OpenApi for ApiDocs {
    fn openapi() -> utoipa::openapi::OpenApi {
        // See <https://docs.rs/utoipa/latest/utoipa/derive.OpenApi.html>
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
            .servers(Some(vec![
                utoipa::openapi::ServerBuilder::new()
                    .url(SELF_URL)
                    .description(Some("Local testing server"))
                    .build(),
                utoipa::openapi::ServerBuilder::new()
                    .url(HOST_URL)
                    .description(Some("Production server"))
                    .build(),
            ]))
            .tags(Some(vec![
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::BUS_INFO_TAG)
                    .description(Some("Battery related endpoints"))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::BUS_DRIVER_TAG)
                    .description(Some("Driver related endpoints"))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::FEATURE_FLAGS_TAG)
                    .description(Some("Feature flags related endpoints"))
                    .build(),
                utoipa::openapi::tag::TagBuilder::new()
                    .name(tags::MAP_RENDERING_TAG)
                    .description(Some("Maps related endpoints"))
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
