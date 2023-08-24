
mod hello_world; 

use axum::{Router, routing::{get,post}} ; 

use hello_world::hello_universe; 

pub fn create_routes() -> Router<>{
    let  router = Router::new().route("/", get(hello_world)).route("/hello", get(|| async { "HELLO"})).route("/universe", post(hello_universe));
    return router;

}

async fn hello_world() -> String {
  "Hello, World".to_string()
}