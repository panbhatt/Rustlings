
use serde::{Serialize, Deserialize};
use axum::Json; 
use axum::http::StatusCode; 
use axum::response::{Response, IntoResponse}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUser {
    username : String, 
    password : String, 
    age : Option<u8>,   //a ge is option here, it wont give error, if we dont pass age, but will give error if username/pwd is empty. 
}

pub async fn validate_user_body(Json(req_user) : Json<RequestUser>) -> Response {
    
    println!("User = {:#?}", req_user); 

    (
        StatusCode::OK, 
        Json(req_user)
    ).into_response()

    
}