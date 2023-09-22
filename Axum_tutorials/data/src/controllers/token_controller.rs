
use axum::{http::StatusCode, extract::Path};
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    exp : usize, 
    iat: usize,
    username: String, 

}



pub async fn create_token( Path(username) : Path<String>) -> Result<String,StatusCode>{

    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes()); 

    let claims = Claims {
        exp : (chrono::Utc::now() + chrono::Duration::seconds(3600)).timestamp() as usize, // EXPIRE 1 hour after. 
        iat : chrono::Utc::now().timestamp() as usize ,
        username : username
    }; 

      let token = encode(&Header::default(), &claims, &key);
        
    if token.is_ok() {
        Ok(token.unwrap())
    } else {
        println!("ERror occured, while creating token = {:#}", token.err().unwrap());
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}