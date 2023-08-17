use once_cell::sync::OnceCell;
//use std::cell::OnceCell;  // Soon once cell would be merged in the main std::cell 
use std::sync::Mutex; 
use chrono::Utc; 

#[derive(Debug)]
struct Logger {
    logs : Mutex<Vec<Log>>, 
    url : String, 
}

#[derive(Debug)]
struct Log {
    message : String, 
    timestamp : u64, 
}

static GLOBAL_LOGGER :  OnceCell<Logger>  = OnceCell::new(); 

fn main() {

    let datadog_url = "http://datadog.com".to_string(); 

    GLOBAL_LOGGER.set(Logger{
        logs : Mutex::new(vec![]), 
        url : datadog_url.clone(), 
    }).unwrap(); 

    println!("Logger = {GLOBAL_LOGGER:#?}"); 

    // Add some logs. 

   GLOBAL_LOGGER.get().unwrap().logs.lock().unwrap().push(Log {
    message : "Pankaj bhatt first Log".into(), 
    timestamp : Utc::now().timestamp() as u64, 
   });

   println!("============== AFTER ADDING LOG ======================="); 
   println!("Logger = {GLOBAL_LOGGER:#?}"); 

    
}