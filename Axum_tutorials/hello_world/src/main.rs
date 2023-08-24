use axum::{
        Router, 
        routing::get
}; 

#[tokio::main]
async fn main() {
    
    let app = Router::new().route("/", get(hello_world)).route("/hello", get(|| async { "HELLO"}));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap(); 

}

async fn hello_world() -> String {
    "Hello World. Pankaj Bhatt".to_owned()
}
