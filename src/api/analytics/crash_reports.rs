use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::{ApiResponse, BurritoAPIError, JsonResult};
use crate::core::AppState;
use crate::features::analytics;
use crate::features::auth::guards::{AuthDriver, StaffUser};
use crate::{docs, router};

router!(
    AnalyticsCrashReportsRouter,
    [
        get_crash_reports,
        post_driver_crash_reports,
        post_users_crash_reports
    ]
);

#[utoipa::path(
    tag = docs::tags::ANALYTICS_TAG,
    description =
        "Get all crash reports that application clients have submitted. This endpoint is only
        available for staff users, as it may contain sensitive information.",
    security(
        ("staff_user_auth" = [])
    )
)]
#[get("/")]
async fn get_crash_reports(
    _user: StaffUser,
    state: &State<AppState>,
) -> ApiResponse<Json<Vec<analytics::entities::CrashReport>>> {
    let reports = analytics::handlers::crash_reports::get_crash_reports_handler(state).await;

    Ok(Json(reports))
}

#[utoipa::path(
    tag = docs::tags::ANALYTICS_TAG,
    description =
        "Submit a bus driver application crash report. Driver app crashes are important to
        monitor because they can affect the bus tracking service.",
    security(
        ("driver_auth" = [])
    ),
    request_body = analytics::entities::CrashReportPayload,
    responses(
        (status = 200, body = analytics::entities::CrashReport),
        (status = 400),
        (status = 401),
    )
)]
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

#[utoipa::path(
    tag = docs::tags::ANALYTICS_TAG,
    description =
        "Submit a client application crash report. Is up to the client to decide what
        errors should be reported to the server for analytics purposes.

        Clients are strongly advised to verify that no sensitive information is sent in the report.
        They may add additional context to the error message.",
    request_body = analytics::entities::CrashReportPayload,
    responses(
        (status = 200, body = analytics::entities::CrashReport),
        (status = 400),
    )
)]
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
