// CODE from https://mixi-developers.mixi.co.jp/how-to-use-type-safe-routing-of-axum-c06c1b1b1ab  

use axum::{
    routing::{get, post} ,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Router,
    Json,
    Extension
};
use sea_orm:: { DatabaseConnection };
use axum_extra::routing::{RouterExt, TypedPath};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;  
use dotenvy_macro::dotenv; 
use datadb::connect_db; 
use datadb::controllers::{
    block_controller::get_block,
    block_controller::create_block, 
};

fn app(dc : DatabaseConnection) -> Router {
    Router::new()
        .route("/api/users/:user_id", get(user_detail))
        .route("/api/blocks/:block_id", get(get_block))
        .route("/api/blocks", post(create_block))
        .layer(Extension(dc))
        .typed_get(user_detail_typed) // THis is the new way to run it. 
}

#[derive(Deserialize, Serialize)]
pub struct PathParam {
    pub user_id: String,
}

#[derive(TypedPath, Deserialize, Serialize)]
#[typed_path("/api/users1/typed/:user_id")]
pub struct PathParamTyped {
    pub user_id: String,
}

pub async fn user_detail(Path(params): Path<PathParam>) -> impl IntoResponse {
    (StatusCode::OK, Json(params)).into_response()
}

pub async fn user_detail_typed(params: PathParamTyped) -> impl IntoResponse {
    (StatusCode::OK, Json(params)).into_response()
}


#[tokio::main]
 async fn main() {
    dotenv().ok(); 

    let db_url = dotenv!("DATABASE_URL"); 
    println!("{}", db_url);
    let dbconn = connect_db(db_url).await.unwrap(); 
    println!("DB CONN = {:#?}", dbconn); 
    run(dbconn).await; 

}


pub async fn run(db : DatabaseConnection) {
    let app_router = app(db); 
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app_router.into_make_service()).await.unwrap() 
}
