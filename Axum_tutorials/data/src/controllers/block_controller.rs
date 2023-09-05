use sea_orm::DatabaseConnection; 
use axum::{Extension} ; 


pub async fn get_block(Extension(db) : Extension<DatabaseConnection>) -> String{
    "Pankj Bhatt - Sample Block".to_string()

}