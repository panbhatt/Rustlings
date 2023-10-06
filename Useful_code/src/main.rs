use log::{self, LevelFilter, info, debug};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;


fn main() {
    println!("INITIAL LOGGER"); 
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("PANKAJ BHATT - INFO");
    debug!("PANKAJ BHATT - DEBUG");
}


