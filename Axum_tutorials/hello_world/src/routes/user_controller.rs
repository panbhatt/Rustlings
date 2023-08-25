

use axum::Json; 
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime} ; 


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User_Create_Request {
    name : String, 
    username : String, 
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User_Create_Response {
    username : String, 
    createdAt : String, 
}


pub async fn post_user_data(body : String) -> String{
    println!("RECEIVING THE BODY = {}", body); 
    body
}

pub async fn post_user_data_json(Json(body) : Json<User_Create_Request>) -> Json<User_Create_Response>{
    dbg!(body.clone()) ; 
    let response = User_Create_Response {
        username : body.username, 
        createdAt : Utc::now().to_rfc2822() 

    };

    Json(response)
    
}

