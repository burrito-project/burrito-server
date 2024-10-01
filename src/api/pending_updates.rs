use rocket::response::status;
use rocket::serde::json::Value;
use rocket::{http::Status, State};
use serde_json::json;

use crate::core::responses;
use crate::entities::AppState;
use crate::schemas;

pub fn routes() -> Vec<rocket::Route> {
    routes![pending_updates, options]
}

fn is_valid_semver<S: Into<String>>(semver: S) -> bool {
    let re = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(semver.into().as_str())
}

#[get("/?<version>&<platform>")]
async fn pending_updates(
    version: Option<String>,
    platform: Option<String>,
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

    let user_platform: schemas::PlatformType = match platform {
        Some(p) => match schemas::PlatformType::try_from(p.as_str()) {
            Ok(p) => p,
            Err(_) => {
                return Err(status::Custom(
                    Status::BadRequest,
                    responses::error_response(
                        "Invalid platform. Use ?platform=<android|ios|web|all>",
                    ),
                ));
            }
        },
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                responses::error_response(
                    "No platform param provided. Use ?platform=<android|ios|web>",
                ),
            ));
        }
    };

    let app_versions = sqlx::query_as!(
        schemas::AppVersion,
        r#"SELECT *
        FROM app_versions
        WHERE semver > $1 AND (platform = 'all' OR platform = $2) AND should_notify = true
        ORDER BY (semver, release_date) DESC;"#, // first is newer
        user_version,
        user_platform.to_string(),
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

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

#[options("/")]
pub fn options() -> Status {
    Status::Ok
}
