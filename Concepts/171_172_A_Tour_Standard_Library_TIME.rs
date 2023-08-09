// Standard Time.

// Use external CHRONO if you want a full fledged time library.

use std::time::{Duration, Instant};
use std::thread::sleep; 

fn main() {
    let time1 = Instant::now();

    println!("{:?}", time1); // Print Instant { tv_sec: 20022, tv_nsec: 565173496 }

    let time2 = Instant::now();

    println!("{:?}", time2 - time1);

    // Use sleep to let the thread sleep. 
    let three_seconds = Duration::from_secs(3); 

    println!("Sleeping now ");
    sleep(three_seconds);
    println!(" I am awake now"); 

}
