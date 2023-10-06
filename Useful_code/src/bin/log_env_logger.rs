use env_logger::{Builder, Target};
use log::{debug, error, info, trace, warn};
use std::env;

// This one is not seems to be working: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html

fn main() {
    /*let mut builder = Builder::new();
    //builder.target(Target::Stdout);
    builder.parse_env(&env::var("MY_RUST_LOG").unwrap_or_default()).init();


        //println!("{}", env::var("MY_RUST_LOG").unwrap_or_default());
    // RUST_LOG CAN BE ANY EN VVARIABLE.

    info!("PANKAJ BHATT");
    log::warn!("WARNING PANKAJ BHATT");
    debug!("DEBUB PANKAJ BHATT");
    error!("ERROR PANKAJ BHATT");
    trace!("ERROR PANKAJ BHATT");

    call_me();*/

    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .init();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");


}
fn call_me() {
    info!("Call me -> INFO");
}

// https://www.youtube.com/watch?v=7EYDaSNSkbM
