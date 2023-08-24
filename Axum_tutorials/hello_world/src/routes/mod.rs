
mod hello_world; 
mod user_controller; 

use axum::{Router, routing::{get,post}} ; 

use hello_world::hello_universe; 
use user_controller::{post_user_data, post_user_data_json }; 

pub fn create_routes() -> Router<>{
    let  router = Router::new()
    .route("/", get(hello_world))
    .route("/hello", get(|| async { "HELLO"}))
    .route("/universe", post(hello_universe))
    .route("/user", post(post_user_data))
    .route("/user/create", post(post_user_data_json)); 
    return router;

}

async fn hello_world() -> String {
  "Hello, World".to_string()
}