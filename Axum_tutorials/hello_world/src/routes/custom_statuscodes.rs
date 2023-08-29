use axum::http::StatusCode; 


pub async fn set_custom_status_code() -> Result<(), StatusCode> {

    Err(StatusCode::IM_A_TEAPOT)
    //    Err(StatusCode::BAD_REQUEST)
    //Ok(()), for 200 Status code. 

}