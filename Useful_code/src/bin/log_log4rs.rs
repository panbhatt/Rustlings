use log::{self, debug, info};
use log4rs; 
fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("PANKAJ BHATT - INFO");
    debug!("PANKAJ BHATT - DEBUG");
}
