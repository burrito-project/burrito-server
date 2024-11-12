use rocket::Route;

use crate::core::utils::with_base;

mod crash_reports;

#[derive(utoipa::OpenApi)]
#[openapi(
    nest(
        (path = "/crash_reports", api = crash_reports::AnalyticsCrashReportsRouter),
    )
)]
pub struct AnalyticsRouter;

impl crate::routes::ApiRouter for AnalyticsRouter {
    fn routes() -> Vec<rocket::Route> {
        let crash_reports_routes = with_base(
            crash_reports::AnalyticsCrashReportsRouter::routes(),
            "/crash_reports",
        );

        crash_reports_routes.collect::<Vec<Route>>()
    }
}
