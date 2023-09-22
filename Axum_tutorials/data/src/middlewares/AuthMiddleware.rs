use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::DatabaseConnection;

pub async fn verify_token<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST);
        
    if let Ok(acutal_token_header) = token {
    let actual_token = acutal_token_header.token();
    println!("TOKEN FROM MIDDLEWARE  = {}", actual_token);

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    println!("Actual DB Connection   = {:?}", database);

    if actual_token == "PANK_TOKEN" {
        //add the request, so that you can add it for further processing.
        request.extensions_mut().insert(3434);
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED) // Otherweise we are going to return th 401 ERROR.
    }
}else {
    Err(StatusCode::UNAUTHORIZED) 
}



}
