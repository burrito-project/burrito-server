use rocket::{
    http::Status,
    response::status,
    serde::json::{self, Json},
    Route, State,
};
use serde_json::Value;

use crate::{auth::AuthDriver, core::responses, entities::AppState, features::analytics};

pub fn routes() -> Vec<Route> {
    routes![
        get_crash_reports,
        post_driver_crash_reports,
        post_users_crash_reports
    ]
}

#[get("/")]
async fn get_crash_reports(state: &State<AppState>) -> Result<Value, status::Custom<Value>> {
    analytics::handlers::crash_reports::get_crash_reports_handler(state).await
}

#[post("/", rank = 0, format = "json", data = "<payload>")]
async fn post_driver_crash_reports(
    payload: Result<Json<analytics::entities::CrashReportPayload>, json::Error<'_>>,
    driver: AuthDriver,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
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
    payload: Result<Json<analytics::entities::CrashReportPayload>, json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
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
