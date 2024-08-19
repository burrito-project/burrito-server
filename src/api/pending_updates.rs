use rocket::response::status;
use rocket::serde::json::Value;
use rocket::{http::Status, Route, State};
use serde_json::json;

use crate::core::responses;
use crate::entities::AppState;

pub fn routes() -> Vec<Route> {
    routes![check_version]
}

fn is_valid_semver<S: Into<String>>(semver: S) -> bool {
    let re = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(semver.into().as_str())
}

#[get("/?<version>")]
pub async fn check_version(
    version: Option<String>,
    state: &State<AppState>,
) -> Result<Value, status::Custom<Value>> {
    let user_version = match version {
        Some(semver) => {
            if !is_valid_semver(&semver) {
                return Err(status::Custom(
                    Status::BadRequest,
                    responses::error_response("Invalid version format. Use x.y.z"),
                ));
            }
            semver
        }
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                responses::error_response("No version param provided. Use ?version=<x.y.z>"),
            ));
        }
    };

    let app_versions = sqlx::query!(
        "SELECT *
        FROM app_versions
        WHERE semver > $1
        ORDER BY release_date ASC",
        user_version,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            responses::error_response("No versions found"),
        )
    })?;

    let must_update = app_versions.iter().any(|version| version.is_mandatory);

    Ok(json!({
        "must_update": must_update,
        "versions": app_versions.iter().map(|version| {
            json!({
                "semver": version.semver,
                "banner_url": version.banner_url,
                "is_mandatory": version.is_mandatory,
                "release_date": version.release_date,
                "release_notes": version.release_notes,
            })
        }).collect::<Vec<Value>>(),
    }))
}
