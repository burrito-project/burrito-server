use rocket::{http::Status, response::status, Route, State};

use crate::auth::AuthDriver;
use crate::core::{
    responses,
    types::{ApiResponse, JsonResult},
};
use crate::entities::AppState;
use crate::features::analytics;

pub fn routes() -> Vec<Route> {
    routes![
        get_crash_reports,
        post_driver_crash_reports,
        post_users_crash_reports
    ]
}

#[get("/")]
async fn get_crash_reports(state: &State<AppState>) -> ApiResponse {
    analytics::handlers::crash_reports::get_crash_reports_handler(state).await
}

#[post("/", rank = 0, format = "json", data = "<payload>")]
async fn post_driver_crash_reports(
    payload: JsonResult<'_, analytics::entities::CrashReportPayload>,
    driver: AuthDriver,
    state: &State<AppState>,
) -> ApiResponse {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    analytics::handlers::crash_reports::create_crash_reports_handler(
        driver.bus_name,
        payload,
        state,
    )
    .await
}

#[post("/", rank = 1, format = "json", data = "<payload>")]
async fn post_users_crash_reports(
    payload: JsonResult<'_, analytics::entities::CrashReportPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    analytics::handlers::crash_reports::create_crash_reports_handler(
        "anon_user".into(),
        payload,
        state,
    )
    .await
}
