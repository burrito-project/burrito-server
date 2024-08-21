use rocket::{http::Status, response::status, serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::{core::responses, entities::AppState, schemas};

pub fn routes() -> Vec<Route> {
    routes![list_advertisements, post_advertisements,]
}

#[get("/")]
async fn list_advertisements(state: &State<AppState>) -> Result<Value, status::Custom<Value>> {
    let advertisements =
        sqlx::query_as_unchecked!(schemas::Advertisement, "SELECT * FROM advertisements;")
            .fetch_all(&state.pool)
            .await
            .map_err(|_| {
                status::Custom(
                    Status::InternalServerError,
                    responses::error_response("No advertisements found"),
                )
            })?;

    Ok(json!(advertisements))
}

#[post("/", format = "json", data = "<payload>")]
async fn post_advertisements(
    payload: Result<Json<schemas::AdvertisementPayload>, rocket::serde::json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    let new_advertisement = sqlx::query_as_unchecked!(
        schemas::Advertisement,
        "INSERT INTO advertisements
        (ad_title, ad_type, ad_content, begin_at, end_at, image_url, is_active, priority, target_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *;",
        payload.ad_title,
        payload.ad_type,
        payload.ad_content,
        payload.begin_at,
        payload.end_at,
        payload.image_url,
        payload.is_active,
        payload.priority,
        payload.target_url,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) => status::Custom(
            Status::BadRequest,
            responses::error_response(db_err.to_string()),
        ),
        _ => status::Custom(
            Status::InternalServerError,
            responses::error_response("Failed to create advertisement"),
        ),
    })?;

    Ok(json!(new_advertisement))
}
