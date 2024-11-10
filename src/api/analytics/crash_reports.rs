use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::core::types::{ApiResponse, BurritoAPIError, JsonResult};
use crate::core::AppState;
use crate::features::analytics;
use crate::features::auth::guards::{AuthDriver, StaffUser};

pub fn routes() -> Vec<Route> {
    routes![
        get_crash_reports,
        post_driver_crash_reports,
        post_users_crash_reports
    ]
}

#[get("/")]
async fn get_crash_reports(
    _user: StaffUser,
    state: &State<AppState>,
) -> ApiResponse<Json<Vec<analytics::entities::CrashReport>>> {
    let reports = analytics::handlers::crash_reports::get_crash_reports_handler(state).await;

    Ok(Json(reports))
}

#[post("/", rank = 0, format = "json", data = "<payload>")]
async fn post_driver_crash_reports(
    payload: JsonResult<'_, analytics::entities::CrashReportPayload>,
    driver: AuthDriver,
    state: &State<AppState>,
) -> ApiResponse<Json<analytics::entities::CrashReport>> {
    if let Err(e) = payload {
        return BurritoAPIError::bad_request(None, e.to_string().into());
    }

    let payload = payload.unwrap().into_inner();

    let new_report = analytics::handlers::crash_reports::create_crash_reports_handler(
        driver.bus_name,
        payload,
        state,
    )
    .await;

    Ok(Json(new_report))
}

#[post("/", rank = 1, format = "json", data = "<payload>")]
async fn post_users_crash_reports(
    payload: JsonResult<'_, analytics::entities::CrashReportPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<analytics::entities::CrashReport>> {
    if let Err(e) = payload {
        return BurritoAPIError::bad_request(None, e.to_string().into());
    }

    let payload = payload.unwrap().into_inner();

    let new_report = analytics::handlers::crash_reports::create_crash_reports_handler(
        "anon_user".into(),
        payload,
        state,
    )
    .await;

    Ok(Json(new_report))
}
