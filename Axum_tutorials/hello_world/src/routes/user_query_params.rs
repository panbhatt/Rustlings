use axum::extract::Query; 
use serde::{Serialize, Deserialize}; 
use axum::Json; 

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    name : String, 
    age : u8, 
}

pub async fn get_user_query_params(Query(userDetails) : Query<QueryParams>)-> Json<QueryParams> {

    println!(" QUery Params -> {userDetails:#?}");
    Json(userDetails)

}