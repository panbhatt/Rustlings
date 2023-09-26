use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

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
    fn into_response(self) -> Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}

pub async fn generate_error_from_error_route<T>(
    req: Request<T>,
    next: Next<T>,
) -> Result<Response, ApiError> {
    println!("The request is - : {}", req.uri());

    if req.uri().to_owned() == "/api/error" {
        Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Unable to process, as it is error route. FROM MIDDLEWARE",
        ))?
    } else {
        Ok(next.run(req).await)
    }
}
