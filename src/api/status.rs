use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::ApiResponse;
use crate::core::AppState;
use crate::features::bus_status;
use crate::{docs, router};

router!(StatusRouter, [get_status, options]);

const DEFAULT_COUNT: usize = 100;

#[utoipa::path(
    tag = docs::tags::BUS_INFO_TAG,
    description = "Get the current bus status, including the last bus stop and the last [count] bus positions.
    Probably the most important endpoint in the whole API, as it provides the most up-to-date information about
    the bus location and the last stop it visited.",
    params(
        ("count", description = "The number of bus position records to retrieve. If not provided, a default value is used")
    ),
    responses(
        (status = 200, body = bus_status::schemas::BurritoStatusResponse)
    )
)]
#[get("/?<count>")]
async fn get_status(
    count: Option<usize>,
    state: &State<AppState>,
) -> ApiResponse<Json<bus_status::schemas::BurritoStatusResponse>> {
    let count = count.unwrap_or(DEFAULT_COUNT);
    let status = bus_status::handlers::get_burrito_status_handler(count, state).await;

    Ok(Json(status))
}

#[utoipa::path(
    tag = docs::tags::BUS_INFO_TAG,
)]
#[options("/")]
fn options() -> Status {
    Status::Ok
}
