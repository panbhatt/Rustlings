pub mod controllers; 
pub mod models; 


use sea_orm::{Database,DatabaseConnection, DbErr} ; 



pub async fn connect_db(db_url : &str) -> Result<DatabaseConnection, DbErr> { 
    let database = Database::connect(db_url).await; 
    println!("Database Connected : {:#?}", database); 
    database
}