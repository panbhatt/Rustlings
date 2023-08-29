
use axum::{ Extension } ; 

use crate::routes::SessionInfo; 


pub async fn get_middleware_msg(Extension(session_info) : Extension<SessionInfo>) -> String {
    String::from(session_info.session_id)
}