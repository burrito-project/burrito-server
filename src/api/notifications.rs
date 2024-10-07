use rocket::{http::Status, response::status, Route, State};

use crate::core::types::{ApiResponse, JsonResult};
use crate::core::{guards::IsMobileChecker, responses};
use crate::core::AppState;
use crate::features::notifications;

pub fn routes() -> Vec<Route> {
    routes![
        get_notifications,
        post_notifications,
        delete_notification,
        options,
    ]
}

#[get("/")]
async fn get_notifications(state: &State<AppState>, is_mobile: IsMobileChecker) -> ApiResponse {
    notifications::handlers::get_notifications_handler(state, is_mobile).await
}

#[post("/", format = "json", data = "<payload>")]
async fn post_notifications(
    payload: JsonResult<'_, notifications::schemas::NotificationPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    notifications::handlers::post_notification_handler(payload, state).await
}

#[delete("/<id>")]
async fn delete_notification(id: i32, state: &State<AppState>) -> ApiResponse {
    notifications::handlers::delete_notification_handler(id, state).await
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
