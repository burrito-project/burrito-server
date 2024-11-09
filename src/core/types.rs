use rocket::serde as rocket_serde;
use serde_json::json;
use strum_macros::Display;

pub type JsonResult<'a, T> = Result<rocket_serde::json::Json<T>, rocket_serde::json::Error<'a>>;

pub type ApiResponse<T: for<'a> rocket::response::Responder<'a, 'a>> = Result<T, BurritoAPIError>;

pub struct UserMessage(pub String);

#[derive(thiserror::Error, Debug, Display)]
pub enum BurritoAPIError {
    BadRequest { user_message: String },
    Unauthorized { user_message: String },
    Database,
    Internal,
}

impl<'r> rocket::response::Responder<'r, 'r> for BurritoAPIError {
    fn respond_to(self, _req: &rocket::Request) -> rocket::response::Result<'r> {
        let mut response = rocket::Response::build();
        let mut res = response.header(rocket::http::ContentType::JSON);

        match self {
            BurritoAPIError::BadRequest { user_message } => {
                res = res.status(rocket::http::Status::BadRequest);
                let json_response: String = json!({ "message": user_message }).to_string();
                res = res.sized_body(json_response.len(), std::io::Cursor::new(json_response));
            }
            BurritoAPIError::Unauthorized { user_message } => {
                res = res.status(rocket::http::Status::Unauthorized);
                let json_response: String = json!({ "message": user_message }).to_string();
                res = res.sized_body(json_response.len(), std::io::Cursor::new(json_response));
            }
            BurritoAPIError::Database => {
                res = res.status(rocket::http::Status::InternalServerError);
            }
            BurritoAPIError::Internal => {
                res = res.status(rocket::http::Status::InternalServerError);
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
