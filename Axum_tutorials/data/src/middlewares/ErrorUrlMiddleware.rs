use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::DatabaseConnection;

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

pub async fn generate_error_from_error_route<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, ApiError> {
    request
    Ok(ApiError::new(StatusCode::UNAUTHORIZED, "Unable to process, as it is error route"))
}
