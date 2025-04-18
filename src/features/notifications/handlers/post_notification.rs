use base64::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

use crate::core::types::{ApiResponse, BurritoAPIError};
use crate::core::AppState;
use crate::features::cdn::{self, ProvideImageService};
use crate::features::notifications::schemas;

const NOTIFICATIONS_PATH: &str = "burrito/notifications/";

pub async fn post_notification_handler(
    payload: schemas::NotificationPayload,
    state: &State<AppState>,
) -> ApiResponse<Json<schemas::Notification>> {
    let image_url: Option<String> = match payload.image_base64 {
        Some(base64_data) => {
            if !base64_data.starts_with("data:image/") {
                return BurritoAPIError::bad_request(Some("Invalid image data"), None);
            }

            let is_png = base64_data.starts_with("data:image/png;base64,");
            let mut base64_data = base64_data;

            if is_png {
                let decoded = BASE64_STANDARD
                    .decode(base64_data.split(",").last().unwrap())
                    .map_err(|e| BurritoAPIError::BadRequest {
                        user_message: format!("Failed to decode image: {:?}", e).into(),
                        error: None,
                    })?;

                let result = oxipng::optimize_from_memory(
                    decoded.as_slice(),
                    &oxipng::Options {
                        fix_errors: true,
                        ..Default::default()
                    },
                )
                .map_err(|e| BurritoAPIError::BadRequest {
                    user_message: format!("Failed to decode image: {:?}", e).into(),
                    error: None,
                })?;

                base64_data = format!(
                    "data:image/png;base64,{}",
                    BASE64_STANDARD.encode(result.as_slice())
                );
            }

            // All notification types accept images so it's not necessary to check the ad_type
            let uploaded_url = cdn::ImageService::upload_image(base64_data, NOTIFICATIONS_PATH)
                .await
                .map_err(|e| BurritoAPIError::BadRequest {
                    user_message: format!("Failed to decode image: {:?}", e).into(),
                    error: Some(e.to_string()),
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
    .unwrap();

    Ok(Json(new_notification))
}
