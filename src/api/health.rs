use rocket::serde::json::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{core::utils::get_uptime, docs, router};

router!(HealthRouter, [ping]);

#[derive(Serialize, ToSchema)]
struct PingResponse {
    #[schema(example = "All systems operational")]
    message: &'static str,
    #[schema(example = "0.1.0")]
    version: &'static str,
    #[schema(example = 1731063831)]
    startup: u64,
    #[schema(example = 69)]
    uptime: u64,
}

#[utoipa::path(
    tag = docs::tags::SERVER_TAG,
    description = "Pongs with server information such as `version`, `startup` and `uptime`.",
    responses(
        (status = 200, body = PingResponse),
    )
)]
#[get("/")]
fn ping() -> Json<PingResponse> {
    let uptime_secs = get_uptime().as_secs();

    Json(PingResponse {
        message: "All systems operational",
        version: env!("CARGO_PKG_VERSION"),
        startup: *crate::startup_unix_timestamp,
        uptime: uptime_secs,
    })
}
