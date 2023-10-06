use log::debug;
use log::error;
use log::info;
use log::warn;
use env_logger; 

// Run it with RUST_LOG=INFO cargo run.

fn main() {
    env_logger::init();
    debug!(" LOG 1 -> Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");

}

// RUN VIA : RUST_LOG=TRACE cargo run --bin log_env_logger  
// RUST_LOG=off -> to disable logging for all modules. 

// For ALL the other packages: 
//RUST_LOG=off,Useful_code=trace
   //Useful_code is the name of our application

//RUST_LOG=errir : Error from all libs and TRACE level for our application. 
