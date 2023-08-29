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

pub async fn get_custom_header(headers:HeaderMap) -> String {

    let msg_value = headers.get("x-message");
    println!("I => {:#?}", msg_value);
    if let Some(hdr_value) = msg_value {
        hdr_value.to_str().unwrap_or_default().to_string()
    } else {
        String::from("")
    }
}