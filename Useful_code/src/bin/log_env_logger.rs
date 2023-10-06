use env_logger::{ Builder, Target};
use log::{debug, error, info, trace, warn};
use std::env;

fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.
    builder.parse_env(&env::var("MY_RUST_LOG").unwrap_or_default()).init(); 
        

        println!("{}", env::var("MY_RUST_LOG").unwrap_or_default()); 
    // RUST_LOG CAN BE ANY EN VVARIABLE.

    info!("PANKAJ BHATT");
    warn!("WARNING PANKAJ BHATT");
    debug!("DEBUB PANKAJ BHATT"); 
    error!("ERROR PANKAJ BHATT");

    call_me();
    
}
fn call_me() {
    info!("Call me -> INFO"); 
}

// https://www.youtube.com/watch?v=7EYDaSNSkbM 
