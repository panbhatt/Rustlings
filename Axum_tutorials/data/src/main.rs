// CODE from https://mixi-developers.mixi.co.jp/how-to-use-type-safe-routing-of-axum-c06c1b1b1ab  

use axum::{
    routing::get,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Router,
    Json,
};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::{Deserialize, Serialize};

fn app() -> Router {
    Router::new()
        .route("/api/users/:user_id", get(user_detail))
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
    
    run().await; 

}


pub async fn run() {
    let app_router = app(); 
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app_router.into_make_service()).await.unwrap() 
}
