use log::{self, debug, info};
use log4rs; 
fn main() {
    log4rs::init_file("/learn/github/Rustlings/Useful_code/src/bin/log4rs.yml", Default::default()).unwrap();

    info!("PANKAJ BHATT - INFO");
    debug!("PANKAJ BHATT - DEBUG");
}


//https://medium.com/@nikmas_dev/advanced-logging-in-rust-with-log4rs-2d712bb322de
