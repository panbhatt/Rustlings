use sea_orm::{DatabaseConnection,Set, ActiveModelTrait} ; 
use axum::{Extension, Json} ; 
use axum::extract::Path; 
use axum::http::StatusCode; 
use axum::response::{Response, IntoResponse}; 
use log::{info, warn};
//use crate::models::prelude::Blocks; 
//use super::super::models::blocks; 
use crate::models::{
    blocks
}; 
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBlock {
    pub hash : String, 
    pub tx_count : i64, 
    pub number : i64, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBlock {
    status : String, 
    message : String 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDataResponse {
    pub hash : String, 
    pub tx_count : i64, 
    pub number : i64,
}

// This function will return the 
pub async fn get_block(Extension(db) : Extension<DatabaseConnection>, Path(block_hash) : Path<String>) -> Response {
    
    info!(">> get_block function with arguments -> {}", block_hash); 

    ( StatusCode::OK, 
        Json(BlockDataResponse{
         hash : "asdasfd".to_owned(), 
         tx_count : 10, 
         number : 232332, 
    })).into_response()

}

pub async fn create_block(Extension(db) : Extension<DatabaseConnection>, Json(block_detail) : Json<RequestBlock> ) -> Response {
        let current_time = SystemTime::now();
        let since_the_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards"); 


        let new_block = blocks::ActiveModel {
            hash: Set(block_detail.hash), 
            number: Set(Some(block_detail.number)), 
            tx_count: Set(Some(block_detail.tx_count)),
            // hash : Set("asdadfsd".to_owned()),
            // number : Set(Some(45454534)),
            // tx_count : Set(Some(100)), 
            timestamp: Set(Some(since_the_epoch.as_secs().into())), 
            prev_block_hash : Set(None), 
            ..Default::default()
        }; 

        let result = new_block.insert(&db).await;
        dbg!(result); 
        
    (
        StatusCode::OK, 
        Json(ResponseBlock {
            status : "success".to_owned(), 
            message : "Block created successfully".to_owned(),
        })
    ).into_response()
    
    }

    