use jsonwebtoken::Header;
use rocket::{http::Status, response::status, serde::json::Json, State};
use serde_json::json;

use crate::{
    core::{responses, types::ApiResponse},
    entities::AppState,
    features::auth::{self, consts::JWT_ENCODING_KEY, schemas::JWTClaims},
};

pub async fn user_login_handler(
    payload: Json<auth::schemas::UserLoginPayload>,
    state: &State<AppState>,
) -> ApiResponse {
    if payload.username == *crate::env::ROOT_SECRET || payload.password == *crate::env::ROOT_SECRET
    {
        let token = jsonwebtoken::encode::<JWTClaims>(
            &Header::default(),
            &JWTClaims::root(),
            &JWT_ENCODING_KEY,
        )
        .unwrap();

        return Ok(json!({
            "token": token,
            "user": *auth::consts::ROOT_USER,
        }));
    }

    let maybe_user = sqlx::query_as!(
        auth::schemas::OptAppUser,
        "SELECT * FROM internal.get_auth_user($1::citext, $2)",
        payload.username,
        payload.password,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let user: auth::schemas::AppUser = match maybe_user.try_into() {
        Ok(user) => user,
        Err(_) => {
            return Err(status::Custom(
                Status::Unauthorized,
                responses::error_response("Invalid credentials".to_string()),
            ))
        }
    };

    let token = jsonwebtoken::encode::<JWTClaims>(
        &Header::default(),
        &JWTClaims::new(user.id),
        &JWT_ENCODING_KEY,
    )
    .unwrap();

    Ok(json!({
        "token": token,
        "user": user,
    }))
}
