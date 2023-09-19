use axum::extract::Path;
use axum::extract::Query;
use axum::http::header::GetAll;
use axum::http::request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use log::{info, warn};
use sea_orm::sea_query::Expr;
use sea_orm::sea_query::IntoCondition;
use sea_orm::Condition;
use sea_orm::IntoActiveModel;
use sea_orm::Order;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde_with::SerializeDisplay;
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

//Filters for below code.
#[derive(Deserialize)]
pub struct GetAllBlockQueryParams {
    size: Option<u64>,
    page: Option<u64>,
    txCount: Option<u64>,
}
/**
 * This function will return all the blocks.
 */
pub async fn get_all_blocks(
    Extension(db): Extension<DatabaseConnection>,
    Query(query_params): Query<GetAllBlockQueryParams>,
) -> Response {
    info!("Starting to get the get_all_blocks Data");

    let mut tx_count_filter = Condition::all();

    if let Some(txCount) = query_params.txCount {
        //tx_count_condition =         tx_count_condition.add(Expr::col(Blocks::Column::TxCount).eq(txCount));
        tx_count_filter = tx_count_filter.add(
            Expr::col(Blocks::Column::TxCount)
                .eq(txCount)
                .into_condition(),
        );
    }

    // This line is used to get All Blocks.
    //let all_blocks = BlockEntity::find().all(&db).await.unwrap();

    let all_blocks = BlockEntity::find()
        .filter(tx_count_filter)
        .all(&db)
        .await
        .unwrap();

    // .order_by(Blocks::Column::Timestamp , Order::Asc)
    // .paginate(&db, (page_no as u64) * ( page_size as u64))

    let blocks_response: Vec<BlockDataResponse> = all_blocks
        .iter()
        .map(|blk| BlockDataResponse {
            hash: blk.hash.clone(),
            tx_count: blk.tx_count.unwrap_or(0),
            number: blk.number.unwrap_or(-1),
        })
        .collect();

    (StatusCode::OK, Json(blocks_response)).into_response()
}

/**
 * This function will return all the blocks in pagination.
 */
pub async fn get_all_blocks_pagination(
    Extension(db): Extension<DatabaseConnection>,
    Query(query_params): Query<GetAllBlockQueryParams>,
) -> Response {
    info!("Starting to get the get_all_blocks Data Pagination");

    //check for Page and size
    let mut page_no: u64 = 0;
    let mut page_size: u64 = 0;
    if let Some(pn) = query_params.page {
        page_no = pn;
    }

    if let Some(ps) = query_params.size {
        page_size = ps;
    }

    let blocks_paginator = BlockEntity::find()
        .order_by_asc(Blocks::Column::Hash)
        .paginate(&db, page_size);

    let all_paginated_blocks = blocks_paginator.fetch_page(page_no - 1).await.unwrap();

    let blocks_response: Vec<BlockDataResponse> = all_paginated_blocks
        .iter()
        .map(|blk| BlockDataResponse {
            hash: blk.hash.clone(),
            tx_count: blk.tx_count.unwrap_or(0),
            number: blk.number.unwrap_or(-1),
        })
        .collect();

    (StatusCode::OK, Json(blocks_response)).into_response()
}

#[derive(Deserialize)]
pub struct UpdateBlockRequest {
    pub number: Option<i64>,
    pub tx_count: Option<i64>,
    pub prev_block_hash: Option<String>,
}
/*
ATOMIC Update - Full Object without the PRIMARY KEY i.e. HASH
*/
pub async fn update_block(
    Extension(db): Extension<DatabaseConnection>,
    Path(hash): Path<String>,
    Json(requestBlock): Json<UpdateBlockRequest>,
) -> Result<(), StatusCode> {
    let update_block = Blocks::ActiveModel {
        hash: Set(hash.clone()),
        number: Set(requestBlock.number),
        tx_count: Set(requestBlock.tx_count),
        prev_block_hash: Set(requestBlock.prev_block_hash),
        ..Default::default()
    };

    let mut hash_filter_condition = Condition::all();

    hash_filter_condition = hash_filter_condition.add(
        Expr::col(Blocks::Column::Hash)
            .eq(hash.clone())
            .into_condition(),
    );

    let result = BlockEntity::update(update_block)
        .filter(hash_filter_condition)
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    println!(" RESULT of Update = {:#?}", result);

    Ok(())
}

/**
 * THis is only for PATCH request.
 */
#[derive(Deserialize, Serialize)]
pub struct UpdatePartialBlockRequest {
    pub number: Option<i64>,
    #[serde(
        default , // import for deseraizliation, 
        skip_serializing_if = "Option::is_none", 
        with = "::serde_with::rust::double_option", 
    )]
    pub tx_count: Option<Option<i64>>,
    #[serde(
        default , // import for deseraizliation, 
        skip_serializing_if = "Option::is_none", 
        with = "::serde_with::rust::double_option", 
    )]
    pub prev_block_hash: Option<Option<String>>,
}
/*
This is PATCH Now
PARTIAL Update - just update PREv_block_hash for a block
*/
pub async fn update_partial_block(
    Extension(db): Extension<DatabaseConnection>,
    Path(hash): Path<String>,
    Json(partial_update_request_block): Json<UpdatePartialBlockRequest>,
) -> Result<(), StatusCode> {
    // FInd the BLock first.
    let db_block_result = BlockEntity::find_by_id(hash.clone())
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let db_block_option;
    if db_block_result.is_ok() {
        db_block_option = db_block_result.unwrap();
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut hash_filter_condition = Condition::all();

    hash_filter_condition = hash_filter_condition.add(
        Expr::col(Blocks::Column::Hash)
            .eq(hash.clone())
            .into_condition(),
    );
    let mut db_block_org;

    if let Some(db_block) = db_block_option {
        db_block_org = db_block.into_active_model();
        if let Some(prev_has) = partial_update_request_block.prev_block_hash {
            db_block_org.prev_block_hash = Set(prev_has);
        }

        // Now UPdate
        let update_result = BlockEntity::update(db_block_org)
            .filter(hash_filter_condition)
            .exec(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

        println!("Update Result = {:#?}", update_result);
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(())
}

pub async fn delete_block(
    Extension(db): Extension<DatabaseConnection>,
    Path(hash): Path<String>,
) -> Response {
    let block_result = BlockEntity::find_by_id(hash.clone())
        .one(&db)
        .await
        .unwrap_or_default();
    if let Some(block) = block_result {
        // Found the block.
        println!("Block Details are => {:#?}", block);
        let block_active_model = block.into_active_model();

        let delete_result = BlockEntity::delete(block_active_model)
            .exec(&db)
            .await
            .unwrap();
        println!("Delete Result = {:#?}", delete_result);
        if delete_result.rows_affected == 1 {
            (
                StatusCode::OK,
                Json(ResponseBlock {
                    status: "success".to_owned(),
                    message: format!("Block -> {} deleted successfully", hash),
                }),
            )
                .into_response()
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseBlock {
                    status: "error".to_owned(),
                    message: format!("An Error occured, while deleting Block -> {} ", hash),
                }),
            )
                .into_response()
        }
    } else {
        println!("Not found block with hash = {}", hash).clone();
        (
            StatusCode::NOT_FOUND,
            Json(ResponseBlock {
                status: "error".to_owned(),
                message: format!("Block -> {} not found successfully", hash.clone()),
            }),
        )
            .into_response()
    }

    // ANOTHER WAY IS
    //let result =BlockEntity::delete_by_id(hash.clone()).exec(&db).await.unwrap_or_default();

    //Another way, to delete many.
    /*
    let mut hash_filter_condition = Condition::all();

    hash_filter_condition = hash_filter_condition.add(
        Expr::col(Blocks::Column::Hash)
            .eq(hash.clone())
            .into_condition()
    );
    let result = BlockEntity::delete_many().filter(hash_filter_condition).exec(&db).await.unwrap_or_default();
    */
}
