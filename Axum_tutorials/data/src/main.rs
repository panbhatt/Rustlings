// CODE from https://mixi-developers.mixi.co.jp/how-to-use-type-safe-routing-of-axum-c06c1b1b1ab

use axum::{
    extract::Path,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, patch, post, put},
    Extension, Json, Router,
};
use axum_extra::routing::{RouterExt, TypedPath};
use datadb::connect_db;
use datadb::controllers::{
    account_controller::create_account, block_controller::create_block,
    block_controller::delete_block, block_controller::get_all_blocks,
    block_controller::get_all_blocks_pagination, block_controller::get_block,
    block_controller::update_block, block_controller::update_partial_block,
    login_controller::login, login_controller::logout, token_controller::create_token,token_controller::verify_token as verify_tk,
    error_controller::return_api_error, 
};

use datadb::middlewares::AuthMiddleware::verify_token;
use datadb::middlewares::ErrorUrlMiddleware::generate_error_from_error_route; 
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use env_logger;
use log::{error, info, warn};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

fn app(dc: DatabaseConnection) -> Router {
    Router::new()
        .route("/api/account", post(create_account)) // Remember it works invert & portected by guard.
        .route_layer(middleware::from_fn(verify_token))
        .route("/api/error", get(return_api_error))
        .route_layer(middleware::from_fn(generate_error_from_error_route))
        .route("/api/users/:user_id", get(user_detail))
        .route("/api/blocks/paginate", get(get_all_blocks_pagination))
        .route("/api/blocks/:block_id", get(get_block))
        .route("/api/blocks/:block_id", delete(delete_block))
        .route("/api/blocks/update/:hash", put(update_block))
        .route("/api/blocks/update/:hash", patch(update_partial_block))
        .route("/api/blocks", get(get_all_blocks))
        .route("/api/blocks", post(create_block))
        .route("/api/login", post(login))
        .route("/api/logout", get(logout))
        .route("/api/token/:username", get(create_token))
        .route("/api/token/verify/:token", get(verify_tk))
        
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
    env_logger::init();
    info!("Application is starting up ");
    dotenv().ok();

    let db_url = dotenv!("DATABASE_URL");
    println!("{}", db_url);
    let dbconn = connect_db(db_url).await.unwrap();
    println!("DB CONN = {:#?}", dbconn);
    run(dbconn).await;
}

pub async fn run(db: DatabaseConnection) {
    let app_router = app(db);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app_router.into_make_service())
        .await
        .unwrap()
}
