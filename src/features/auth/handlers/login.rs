use jsonwebtoken::Header;
use rocket::{serde::json::Json, State};

use crate::core::types::ApiResponse;
use crate::core::types::BurritoAPIError;
use crate::core::AppState;
use crate::features::auth::{self, consts::JWT_ENCODING_KEY, schemas::JWTClaims};

pub async fn user_login_handler(
    payload: Json<auth::schemas::UserLoginPayload>,
    state: &State<AppState>,
) -> ApiResponse<Json<auth::schemas::UserLoginResponse>> {
    if payload.username == *crate::env::ROOT_SECRET || payload.password == *crate::env::ROOT_SECRET
    {
        let token = jsonwebtoken::encode::<JWTClaims>(
            &Header::default(),
            &JWTClaims::root(),
            &JWT_ENCODING_KEY,
        )
        .unwrap();

        return Ok(Json(auth::schemas::UserLoginResponse {
            token,
            user: auth::consts::ROOT_USER.clone(),
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
            return Err(BurritoAPIError::Unauthorized {
                user_message: Some("Invalid credentials".into()),
            })
        }
    };

    let token = jsonwebtoken::encode::<JWTClaims>(
        &Header::default(),
        &JWTClaims::new(user.id),
        &JWT_ENCODING_KEY,
    )
    .unwrap();

    Ok(Json(auth::schemas::UserLoginResponse { token, user }))
}
