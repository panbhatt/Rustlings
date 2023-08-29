use axum::{
    http :: { Request, StatusCode},
    response:: {Response } , 
    middleware :: {Next} 
}; 

use super::middleware_custom_handler::HeaderMessage; 

pub async fn set_auth_middleware<B>(mut request: Request<B>, next : Next<B>) -> Result<Response, StatusCode> {

        let headers  = request.headers(); 
        let message = headers.get("authcode").ok_or_else(|| StatusCode::BAD_REQUEST)?; 
        let msg = message.to_str().map_err(|_| StatusCode::BAD_REQUEST)?.to_owned();
        let extensions = request.extensions_mut(); 
        extensions.insert(HeaderMessage(msg.to_owned())); 
        println!("Automatically set AUTH MIDDLEWARE: "); 
    Ok(next.run(request).await)
}