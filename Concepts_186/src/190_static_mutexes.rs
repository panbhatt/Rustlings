use chrono::Utc;
use std::sync::Mutex;

// Static Mutexes.

#[derive(Debug)]
struct Log {
    date: i64,
    message: String,
}

static GLOBAL_LOGGER: Mutex<Vec<Log>> = Mutex::new(Vec::new()); // Static Mutex Declaration. 

fn add_logs(msg: String) {
    GLOBAL_LOGGER.lock().unwrap().push(Log {
        date: Utc::now().timestamp(),
        message: msg,
    });
}

fn main() {
    add_logs("hello world".into());
    add_logs("hello universe".into());
    println!("{GLOBAL_LOGGER:#?}");
}
