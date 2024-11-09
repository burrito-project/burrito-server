use rocket::serde as rocket_serde;
use serde_json::json;
use strum_macros::Display;

pub type JsonResult<'a, T> = Result<rocket_serde::json::Json<T>, rocket_serde::json::Error<'a>>;

pub type ApiResponse<T: for<'a> rocket::response::Responder<'a, 'a>> = Result<T, BurritoAPIError>;

pub struct UserMessage(pub String);

#[derive(thiserror::Error, Debug, Display)]
pub enum BurritoAPIError {
    BadRequest {
        user_message: Option<String>,
        error: Option<String>,
    },
    Unauthorized {
        user_message: Option<String>,
    },
    Forbbiden {
        user_message: Option<String>,
    },
    NotFound {
        user_message: Option<String>,
    },
    Database,
    Internal,
}

impl BurritoAPIError {
    pub fn not_found<S: Into<String>, T>(msg: S) -> ApiResponse<T> {
        Err(BurritoAPIError::NotFound {
            user_message: Some(msg.into()),
        })
    }

    pub fn unauthorized<S: Into<String>, T>(msg: S) -> ApiResponse<T> {
        Err(BurritoAPIError::Unauthorized {
            user_message: Some(msg.into()),
        })
    }

    pub fn forbbiden<S: Into<String>, T>(msg: S) -> ApiResponse<T> {
        Err(BurritoAPIError::Forbbiden {
            user_message: Some(msg.into()),
        })
    }
}

impl<'r> rocket::response::Responder<'r, 'r> for BurritoAPIError {
    fn respond_to(self, _req: &rocket::Request) -> rocket::response::Result<'r> {
        let mut response = rocket::Response::build();
        let mut res = response.header(rocket::http::ContentType::JSON);

        match self {
            BurritoAPIError::BadRequest {
                user_message,
                error,
            } => {
                res = res.status(rocket::http::Status::BadRequest);
                let json_response: String =
                    json!({ "message": user_message, "error": error }).to_string();
                res = res.sized_body(json_response.len(), std::io::Cursor::new(json_response));
            }
            BurritoAPIError::Unauthorized { user_message } => {
                res = res.status(rocket::http::Status::Unauthorized);
                let json_response: String =
                    json!({ "message": user_message, "error": null }).to_string();
                res = res.sized_body(json_response.len(), std::io::Cursor::new(json_response));
            }
            BurritoAPIError::Database => {
                res = res.status(rocket::http::Status::InternalServerError);
            }
            BurritoAPIError::Internal => {
                res = res.status(rocket::http::Status::InternalServerError);
            }
            BurritoAPIError::NotFound { user_message } => {
                res = res.status(rocket::http::Status::NotFound);
                let json_response: String =
                    json!({ "message": user_message, "error": null }).to_string();
                res = res.sized_body(json_response.len(), std::io::Cursor::new(json_response));
            }
            BurritoAPIError::Forbbiden { user_message } => {
                res = res.status(rocket::http::Status::NotFound);
                let json_response: String =
                    json!({ "message": user_message, "error": null }).to_string();
                res = res.sized_body(json_response.len(), std::io::Cursor::new(json_response));
            }
        }

        Ok(res.finalize())
    }
}

/*
return Err(status::Custom(
    Status::Unauthorized,
    responses::error_response("Invalid credentials".to_string()),
))
*/
