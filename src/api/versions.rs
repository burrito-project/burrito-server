use rocket::{http::Status, response::status, serde::json::Json, Route, State};
use serde_json::{json, Value};

use crate::{
    core::responses,
    entities::AppState,
    schemas::{self, AppVersionPayload},
};

pub fn routes() -> Vec<Route> {
    routes![list_app_versions, post_app_versions,]
}

#[get("/")]
async fn list_app_versions(state: &State<AppState>) -> Result<Value, status::Custom<Value>> {
    let versions = sqlx::query_as!(schemas::AppVersion, "SELECT * FROM app_versions;")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            status::Custom(
                Status::InternalServerError,
                responses::error_response("No versions found"),
            )
        })?;

    Ok(json!(versions))
}

#[post("/", format = "json", data = "<payload>")]
async fn post_app_versions(
    payload: Result<Json<AppVersionPayload>, rocket::serde::json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    let new_version = sqlx::query_as!(
        schemas::AppVersion,
        "INSERT INTO app_versions
        (semver, banner_url, is_mandatory, platform, release_date, release_notes)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *;",
        payload.semver,
        payload.banner_url,
        payload.is_mandatory,
        payload.platform.to_string(),
        payload.release_date,
        payload.release_notes,
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
            responses::error_response(format!("Failed to create version: {e}")),
        ),
    })?;

    Ok(json!(new_version))
}
