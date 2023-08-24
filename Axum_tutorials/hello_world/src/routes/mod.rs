use axum::{Router, routing::get, body::Body} ; 

pub fn create_routes() -> Router<>{
    let router = Router::new().route("/", get(hello_world)).route("/hello", get(|| async { "HELLO"}));;
    //router.route("/hello", get(|| async { "HELLO"})); 
    return router;

}

async fn hello_world() -> String {
    "Hello World. Pankaj Bhatt".to_owned()
}