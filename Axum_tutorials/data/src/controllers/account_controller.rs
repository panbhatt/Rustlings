use axum::extract::Path;
use axum::extract::Query;
use axum::http::header::GetAll;
use axum::http::request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseUser {
    username: String,
    id: isize,
}

pub async fn create_account(
    Extension(db): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
    //Extension(age): Extension<isize>,
) -> Response {
    print!("Request User = {:?}", request_user);
    //println!("I SIZE FROM THE AUTH HEADER IS = {}", age);

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ResponseUser {
            username: request_user.username.to_owned(),
            id: 34,
        }),
    )
        .into_response()

    // CHecedk block_controller for inserting in the Database.
}
