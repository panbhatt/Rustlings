use sea_orm::{DatabaseConnection,Set, ActiveModelTrait} ; 
use axum::{Extension} ; 
//use crate::models::prelude::Blocks; 
//use super::super::models::blocks; 
use crate::models::{
    blocks
}; 
use std::time::{Duration, SystemTime, UNIX_EPOCH};




pub async fn get_block(Extension(db) : Extension<DatabaseConnection>) -> String{
    "Pankj Bhatt - Sample Block".to_string()

}

pub async fn create_block(Extension(db) : Extension<DatabaseConnection>) -> String{
    let current_time = SystemTime::now();
    let since_the_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards"); 


    let new_block = blocks::ActiveModel {
        hash: Set("PANKAJ_BHATT".to_owned()), 
        number: Set(Some(99999999)), 
        tx_count: Set(Some(11)),
        timestamp: Set(Some(since_the_epoch.as_secs().into())), 
        prev_block_hash : Set(None), 
        ..Default::default()
    }; 

    let result = new_block.insert(&db).await;
    dbg!(result); 
    "ABC".to_owned()
}