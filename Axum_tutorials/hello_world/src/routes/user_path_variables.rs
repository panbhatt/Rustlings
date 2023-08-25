
use axum::extract::Path; 


pub async fn get_user(Path(id) : Path<i32>) -> String {

    id.to_string()
}