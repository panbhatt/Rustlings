use axum::{extract::Path, http::StatusCode};
use dotenvy_macro::dotenv;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    exp: usize,
    iat: usize,
    username: String,
}

pub async fn create_token(Path(username): Path<String>) -> Result<String, StatusCode> {
    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());

    let claims = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::seconds(3600)).timestamp() as usize, // EXPIRE 1 hour after.
        iat: chrono::Utc::now().timestamp() as usize,
        username: username,
    };

    let token = encode(&Header::default(), &claims, &key);

    if token.is_ok() {
        Ok(token.unwrap())
    } else {
        println!(
            "ERror occured, while creating token = {:#}",
            token.err().unwrap()
        );
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn verify_token(Path(token): Path<String>) -> Result<String, StatusCode> {

    let secret = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    println!("TOKEN = {}", token); 

    let result = decode::<Claims>(token.as_str(), &key, &Validation::new(Algorithm::HS256));
    

    if result.is_ok() {
        Ok("Correct Token ".to_string())
    } else {
        println!("JSON TOKEN PARSING ERROR ");
        match result.err().unwrap().kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(StatusCode::BAD_REQUEST),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
