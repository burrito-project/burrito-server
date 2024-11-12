use rocket::serde::json::Json;
use rocket::{http::Status, State};

use crate::core::types::{ApiResponse, BurritoAPIError, JsonResult};
use crate::core::AppState;
use crate::features::auth::guards::StaffUser;
use crate::features::notifications;
use crate::{docs, router};

router!(
    NotificationsRouter,
    [
        get_notifications,
        post_notifications,
        delete_notification,
        options
    ]
);

#[utoipa::path(
    tag = docs::tags::NOTIFICATIONS_TAG,
    description =
        "Lists all the **active** app notifications available.
        Meant to be called from the clients at a regular interval.",
    responses(
        (status = 200, body = Vec<notifications::schemas::Notification>),
    )
)]
#[get("/")]
async fn get_notifications(
    state: &State<AppState>,
) -> Json<Vec<notifications::schemas::Notification>> {
    Json(notifications::handlers::get_notifications_handler(state).await)
}

#[utoipa::path(
    tag = docs::tags::NOTIFICATIONS_TAG,
    request_body = notifications::schemas::NotificationPayload,
    description =
        "Endpoint for admins to post notifications to the app. The notifications will be
        displayed in the app to all users. Notifications are fully customizable.
        See the payload schema for more information.",
    responses(
        (status = 200, body = notifications::schemas::Notification),
        (status = 400, body = String),
        (status = 401, body = String),
    )
)]
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

#[utoipa::path(
    tag = docs::tags::NOTIFICATIONS_TAG,
    description =
        "Deletes a notification by its id. If you only want to hide a notification,
        set the `is_active` field to `false` instead.",
)]
#[delete("/<id>")]
async fn delete_notification(
    _user: StaffUser,
    id: i32,
    state: &State<AppState>,
) -> ApiResponse<Json<notifications::schemas::Notification>> {
    notifications::handlers::delete_notification_handler(id, state).await
}

#[utoipa::path(
    tag = docs::tags::NOTIFICATIONS_TAG,
)]
#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
