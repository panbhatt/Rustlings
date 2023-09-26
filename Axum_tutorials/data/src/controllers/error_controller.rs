use axum::{response::{IntoResponse, Response}, http::{ StatusCode}, Json};
use serde::{Deserialize, Serialize};



pub struct ApiError {
    code: StatusCode,
    message: String,
}

#[derive(Serialize, Clone)]
pub struct ResponseMessage {
    message: String,
}

impl ApiError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}


pub  async fn return_api_error() -> Response{
    /*ApiError {
        code : StatusCode::INTERNAL_SERVER_ERROR, 
        message : "CUSTOM ERROR ".to_string(),
    }*/

    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "CUSTOM ERROR FROM HANDLER").into_response()
  /*(StatusCode::BAD_REQUEST, 
     Json(ResponseMessage {
        message : "HELLO".to_string()
     })).into_response()
     */
    //Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "CUSTOM ERROR"))
}