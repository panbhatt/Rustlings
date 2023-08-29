use axum::http::StatusCode; 
use axum::response::{Response, IntoResponse}; 

pub async fn set_custom_201() -> Response {

    (
        StatusCode::CREATED, 
        "Sample Custom Status Created".to_owned()
    ).into_response()
}