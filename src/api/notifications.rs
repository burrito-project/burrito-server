use rocket::serde::json::Json;
use rocket::{http::Status, Route, State};

use crate::core::types::{ApiResponse, BurritoAPIError, JsonResult};
use crate::core::AppState;
use crate::docs;
use crate::features::auth::guards::StaffUser;
use crate::features::notifications;

pub fn routes() -> Vec<Route> {
    routes![
        get_notifications,
        post_notifications,
        delete_notification,
        options,
    ]
}

#[utoipa::path(
    tag = docs::tags::NOTIFICATIONS_TAG,
    responses(
        (
            status = 200,
            description = "Lists all the active app notifications available. Meant to be directly called from the clients.",
            body = Vec<notifications::schemas::Notification>,
        ),
    )
)]
#[get("/")]
async fn get_notifications(
    state: &State<AppState>,
) -> Json<Vec<notifications::schemas::Notification>> {
    Json(notifications::handlers::get_notifications_handler(state).await)
}

#[post("/", format = "json", data = "<payload>")]
async fn post_notifications(
    _user: StaffUser,
    payload: JsonResult<'_, notifications::schemas::NotificationPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<notifications::schemas::Notification>> {
    if let Err(e) = payload {
        return BurritoAPIError::bad_request(None, Some(e.to_string()));
    }

    let payload = payload.unwrap().into_inner();

    notifications::handlers::post_notification_handler(payload, state).await
}

#[delete("/<id>")]
async fn delete_notification(
    _user: StaffUser,
    id: i32,
    state: &State<AppState>,
) -> ApiResponse<Json<notifications::schemas::Notification>> {
    notifications::handlers::delete_notification_handler(id, state).await
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
