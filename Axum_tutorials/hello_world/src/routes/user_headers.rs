use axum::{
    extract::{Json, Path, Extension, Query}, 
    http::{Request, header::HeaderMap}, 
    TypedHeader,
    headers::UserAgent

}; 



pub async fn get_user_headers(headers: HeaderMap)  -> String{
    println!("{:#?}", headers); 
    format!("{:?}", headers)
}

pub async fn get_user_agent(TypedHeader(user_agent) : TypedHeader<UserAgent>) -> String {

    let ua = user_agent.to_string(); 
    println!("User-Agent: {}", ua);
    ua
}