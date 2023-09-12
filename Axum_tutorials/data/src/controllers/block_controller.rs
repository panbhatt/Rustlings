use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
//use crate::models::prelude::Blocks;
//use super::super::models::blocks;

//use crate::models::{blocks, blocks::Entity as BlockEntity}; DOes not work.
use super::super::models::blocks as Blocks;
use super::super::models::blocks::Entity as BlockEntity;

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBlock {
    pub hash: String,
    pub tx_count: i64,
    pub number: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseBlock {
    status: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDataResponse {
    pub hash: String,
    pub tx_count: i64,
    pub number: i64,
}

// This function will return the
pub async fn get_block(
    Extension(db): Extension<DatabaseConnection>,
    Path(block_hash): Path<String>,
) -> Response {
    info!(
        ">> get_block function with arguments -> {}",
        block_hash.clone()
    );

    //let block_db = BlockEntity::find_by_id(&block_hash).one(&db).await;
    //let all_blocks = blocks::Entity::find().all(&db).await.unwrap();
    //let block_db = blocks::find_by_id(block_hash).one(&db).await.unwrap();
    let block_db = BlockEntity::find_by_id(block_hash.clone())
        .one(&db)
        .await
        .unwrap();

    if let Some(block) = block_db {
        (
            StatusCode::OK,
            Json(BlockDataResponse {
                hash: block.hash,
                tx_count: 10,
                number: 232332,
            }),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ResponseBlock {
                status: "NOT_FOUND".to_owned(),
                message: format!("Block Hash -> {} not found", block_hash.clone()),
            }),
        )
            .into_response()
    }
}

pub async fn create_block(
    Extension(db): Extension<DatabaseConnection>,
    Json(block_detail): Json<RequestBlock>,
) -> Response {
    let current_time = SystemTime::now();
    let since_the_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let new_block = Blocks::ActiveModel {
        hash: Set(block_detail.hash),
        number: Set(Some(block_detail.number)),
        tx_count: Set(Some(block_detail.tx_count)),
        // hash : Set("asdadfsd".to_owned()),
        // number : Set(Some(45454534)),
        // tx_count : Set(Some(100)),
        timestamp: Set(Some(since_the_epoch.as_secs().into())),
        prev_block_hash: Set(None),
        ..Default::default()
    };

    let result = new_block.insert(&db).await;
    dbg!(result);

    (
        StatusCode::OK,
        Json(ResponseBlock {
            status: "success".to_owned(),
            message: "Block created successfully".to_owned(),
        }),
    )
        .into_response()
}
