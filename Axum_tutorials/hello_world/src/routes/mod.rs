
mod hello_world; 
mod user_controller; 
mod user_path_variables; 
mod user_query_params; 
mod user_headers; 

use axum::{Router, routing::{get,post}} ; 

use hello_world::hello_universe; 
use user_controller::{post_user_data, post_user_data_json }; 
use user_path_variables::{get_user}; 
use user_query_params::{get_user_query_params};
use user_headers::{get_user_headers, get_user_agent,get_custom_header} ;


pub fn create_routes() -> Router<>{
    let  router = Router::new()
    .route("/", get(hello_world))
    .route("/hello", get(|| async { "HELLO"}))
    .route("/universe", post(hello_universe))
    .route("/user", post(post_user_data))
    .route("/user/query", get(get_user_query_params))
    .route("/user/headers", get(get_user_headers) )
    .route("/user/headers/user_agent", get(get_user_agent) )
    .route("/user/headers/custom", get(get_custom_header) )
    .route("/user/:id", get(get_user))
    .route("/user/create", post(post_user_data_json)); 
    return router;

} 
async fn hello_world() -> String {
  "Hello, World".to_string()
}