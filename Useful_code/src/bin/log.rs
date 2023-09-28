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
