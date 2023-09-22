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
    let hashed_pwd = hash_password(request_user.password.clone()).unwrap();
    println!("Hashed Password is = {}", hashed_pwd);
    println!(
        "Verified Hashed Password is = {:#?}",
        verify_password(request_user.password.clone(), hashed_pwd.as_str())
    );

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

fn hash_password(password: String) -> Result<String, String> {
    let hashed_pwd = bcrypt::hash(password, 14);
    if let Ok(hp) = hashed_pwd {
        Ok(hp)
    } else {
        Err(hashed_pwd.unwrap())
    }
}

fn verify_password(password: String, hashed_pwd: &str) -> Result<bool, String> {
    let result = bcrypt::verify(password, hashed_pwd);
    if result.is_err() {
        Err(result.err().unwrap().to_string())
    } else {
        Ok(result.unwrap())
    }
}
