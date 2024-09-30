use base64::prelude::*;
use rocket::{http::Status, response::status, serde::json, serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::{
    core::responses,
    entities::AppState,
    features::{
        cdn::{self, ProvideImageService},
        flags,
    },
    schemas,
};

const NOTIFICATIONS_PATH: &str = "burrito/notifications/";

pub fn routes() -> Vec<Route> {
    routes![
        list_notifications,
        post_notifications,
        delete_notification,
        options,
    ]
}

#[get("/")]
async fn list_notifications(state: &State<AppState>) -> Result<Value, status::Custom<Value>> {
    let random_order = flags::utils::get_flag(&state.pool, "random_ads", true).await;

    return match random_order {
        true => {
            let notifications = sqlx::query_as!(
                schemas::Notification,
                "SELECT * FROM notification_ads ORDER BY RANDOM();",
            )
            .fetch_all(&state.pool)
            .await
            .map_err(|_| {
                status::Custom(
                    Status::InternalServerError,
                    responses::error_response("No notifications found"),
                )
            })?;

            Ok(json!(notifications))
        }
        false => {
            let notifications = sqlx::query_as!(
                schemas::Notification,
                "SELECT * FROM notification_ads ORDER BY priority ASC;",
            )
            .fetch_all(&state.pool)
            .await
            .map_err(|_| {
                status::Custom(
                    Status::InternalServerError,
                    responses::error_response("No notifications found"),
                )
            })?;

            Ok(json!(notifications))
        }
    };
}

#[post("/", format = "json", data = "<payload>")]
async fn post_notifications(
    payload: Result<Json<schemas::NotificationPayload>, json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    let image_url: Option<String> = match payload.image_base64 {
        Some(base64_data) => {
            if !base64_data.starts_with("data:image/") {
                return Err(status::Custom(
                    Status::BadRequest,
                    responses::error_response("Invalid image data"),
                ));
            }

            let is_png = base64_data.starts_with("data:image/png;base64,");
            let mut base64_data = base64_data;

            if is_png {
                let decoded = BASE64_STANDARD
                    .decode(base64_data.split(",").last().unwrap())
                    .map_err(|e| {
                        status::Custom(
                            Status::BadRequest,
                            responses::error_response(format!("Failed to decode image: {:?}", e)),
                        )
                    })?;

                let result = oxipng::optimize_from_memory(
                    decoded.as_slice(),
                    &oxipng::Options {
                        fix_errors: true,
                        ..Default::default()
                    },
                )
                .map_err(|e| {
                    status::Custom(
                        Status::BadRequest,
                        responses::error_response(format!("Failed to optimize image: {:?}", e)),
                    )
                })?;

                base64_data = format!(
                    "data:image/png;base64,{}",
                    BASE64_STANDARD.encode(result.as_slice())
                );
            }

            // All notification types accept images so it's not necessary to check the ad_type
            let uploaded_url = cdn::ImageService::upload_image(base64_data, NOTIFICATIONS_PATH)
                .await
                .map_err(|e| {
                    status::Custom(
                        Status::BadRequest,
                        responses::error_response(format!("Failed to upload image: {:?}", e)),
                    )
                })?;

            Some(uploaded_url)
        }
        None => None,
    };

    let new_notification = sqlx::query_as!(
        schemas::Notification,
        "INSERT INTO notification_ads
        (is_active, ad_title, ad_type, priority, image_url, target_url, ad_content, begin_at, end_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *;",
        payload.is_active,
        payload.ad_title,
        payload.ad_type.to_string(),
        payload.priority,
        image_url,
        payload.target_url,
        payload.ad_content,
        payload.begin_at,
        payload.end_at,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) => status::Custom(
            Status::BadRequest,
            responses::error_response(db_err.to_string()),
        ),
        e => status::Custom(
            Status::InternalServerError,
            responses::error_response(format!("Failed to create notification: {}", e)),
        ),
    })?;

    Ok(json!(new_notification))
}

#[delete("/<id>")]
async fn delete_notification(
    id: i32,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    let deleted_notification = sqlx::query_as!(
        schemas::Notification,
        "DELETE FROM notification_ads WHERE id = $1 RETURNING *;",
        id,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("Failed to delete notification"),
        )
    })?;

    Ok(json!(deleted_notification))
}

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
