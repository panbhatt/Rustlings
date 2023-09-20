use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Extension, Json};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginUserRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserResponse {
    status: String,
    token: String,
}

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(login_request): Json<LoginUserRequest>,
) -> Response {
    if login_request.username != "pankaj" {
        return (
            StatusCode::NOT_FOUND,
            Json(LoginUserResponse {
                status: "error".to_owned(),
                token: "".to_owned(),
            }),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(LoginUserResponse {
            status: "success".to_owned(),
            token: "LSDFLSDJSD".to_owned(),
        }),
    )
        .into_response()
}
