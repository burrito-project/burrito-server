use rocket::{
    http::Status,
    response::status,
    serde::{self, json::Json},
    Route, State,
};
use serde_json::{json, Value};

use crate::{core::responses, entities::AppState, schemas};

pub fn routes() -> Vec<Route> {
    routes![
        list_app_versions,
        post_app_versions,
        patch_app_version,
        delete_app_version,
    ]
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
                responses::error_response("Error retrieveing versions"),
            )
        })?;

    Ok(json!(versions))
}

#[post("/", format = "json", data = "<payload>")]
async fn post_app_versions(
    payload: Result<Json<schemas::AppVersionPayload>, serde::json::Error<'_>>,
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

#[patch("/<id>", format = "json", data = "<payload>")]
async fn patch_app_version(
    id: i32,
    payload: Result<Json<schemas::AppVersionPatchPayload>, serde::json::Error<'_>>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    if let Err(e) = payload {
        return Err(status::Custom(
            Status::BadRequest,
            responses::error_response(e.to_string()),
        ));
    }

    let payload = payload.unwrap().into_inner();

    // we only set the fields that are not None
    let updated_version = sqlx::query_as!(
        schemas::AppVersion,
        "UPDATE app_versions
        SET semver = COALESCE($2, semver),
            banner_url = COALESCE($3, banner_url),
            is_mandatory = COALESCE($4, is_mandatory),
            should_notify = COALESCE($5, should_notify),
            platform = COALESCE($6, platform),
            release_date = COALESCE($7, release_date),
            release_notes = COALESCE($8, release_notes)
        WHERE id = $1
        RETURNING *;",
        id,
        payload.semver,
        payload.banner_url,
        payload.is_mandatory,
        payload.should_notify,
        payload.platform.map(|p| p.to_string()),
        payload.release_date,
        payload.release_notes,
    );

    let updated_version = updated_version
        .fetch_one(&state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => status::Custom(
                Status::BadRequest,
                responses::error_response(db_err.to_string()),
            ),
            e => status::Custom(
                Status::InternalServerError,
                responses::error_response(format!("Failed to update version: {e}")),
            ),
        })?;

    Ok(json!(updated_version))
}

#[delete("/<id>")]
async fn delete_app_version(
    id: i32,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    let deleted_version = sqlx::query_as!(
        schemas::AppVersion,
        "DELETE FROM app_versions WHERE id = $1 RETURNING *;",
        id,
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
            responses::error_response(format!("Failed to delete version: {e}")),
        ),
    })?;

    Ok(json!(deleted_version))
}
