use axum::Json; 
use serde::{Serialize, Deserialize}; 
use axum::response::{Response, IntoResponse}; 
use axum::http::StatusCode; 

#[derive(Debug, Clone, Serialize)]
pub struct Employee {
    name : String, 
    age : u8, 
}

pub async fn get_json() -> Json<Employee>{
    let emp = Employee {
        name: "Pankaj Bhatt".to_owned(),
        age : 38, 
    };

    Json(emp)
}  

pub async fn get_json_with_status() -> Response{
    let emp = Employee {
        name: "Pankaj Bhatt".to_owned(),
        age : 38, 
    };

    (
        StatusCode::CREATED, 
        Json(emp)
    ).into_response()
    
}  