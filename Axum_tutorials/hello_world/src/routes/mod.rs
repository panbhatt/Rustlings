
mod hello_world; 
mod user_controller; 
mod user_path_variables; 
mod user_query_params; 
mod user_headers; 
mod middlewares; 
mod middleware_custom_handler; 
mod auth_middleware; 
mod custom_statuscodes; 

use axum::{Router, routing::{get,post}, Extension} ; 
use axum::{
  http::{Request, header::HeaderMap, Method},
  middleware, 
}; 
use tower_http::cors::{CorsLayer, Any}; 


use hello_world::hello_universe; 
use user_controller::{post_user_data, post_user_data_json }; 
use user_path_variables::{get_user}; 
use user_query_params::{get_user_query_params};
use user_headers::{get_user_headers, get_user_agent,get_custom_header} ;
use middlewares::{get_middleware_msg}; 
use middleware_custom_handler::custom_middleware_handler; 
use auth_middleware::{set_auth_middleware}; 
use custom_statuscodes::set_custom_status_code; 


#[derive(Clone)]
pub struct SessionInfo {
  pub session_id : String
}


pub fn create_routes() -> Router<>{

  // CORS Implementation - comes from tower-http
  let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any); 
    let session_info = SessionInfo {
      session_id : "asdfsa-3434sdfsdf-34dfs3d-34f34f34-f343434".to_string(),
    };

    let  router = Router::new()
    .route("/middleware/custom/header", get(custom_middleware_handler  ))
    .route_layer(middleware::from_fn(set_auth_middleware))
    .route("/", get(hello_world))
    .route("/hello", get(|| async { "HELLO"}))
    .route("/universe", post(hello_universe))
    .route("/user", post(post_user_data))
    .route("/user/query", get(get_user_query_params))
    .route("/user/headers", get(get_user_headers) )
    .route("/user/headers/user_agent", get(get_user_agent) )
    .route("/user/headers/custom", get(get_custom_header) )
    .route("/user/:id", get(get_user))
    .route("/user/create", post(post_user_data_json))
    .route("/middleware/message", get(get_middleware_msg  ))
    .route("/custom/status", get(set_custom_status_code))
    .layer(cors)
    .layer(Extension(session_info));
   
    return router;

} 
async fn hello_world() -> String {
  "Hello, World".to_string()
}