use lazy_static::lazy_static;
use reqwest::Client;

#[derive(Debug)]
struct ErrorListener {
    url: String,
    client: Client,
}

lazy_static! {
// This whole code block is a problem unless wrapped in lazystatic macro as we are trying to generate static constants at runtime which is wrong.
//static ERROR_LISTENER : ErrorListener = ErrorListener {
static ref ERROR_LISTENER : ErrorListener = ErrorListener {
    url : "http://localhost:8080".to_string(),
    client : Client::default(),
};
}

fn main() {
    println!("URL -> {}", ERROR_LISTENER.url);   // Now it can be constant but evaulated lazily. 
}
