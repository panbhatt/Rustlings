

use std::time::{Duration, Instant}; 

fn main() {
    let elapsedTime = get_elapsed_time(); 
    println!("Elapsed time IN seconds-> {}", elapsedTime.as_secs()); // This will roughly print 10 seconds
    println!("Elapsed time IN millis -> {}", elapsedTime.as_millis()); // This will roughly print ~10000 milli seconds
}


// This function will return ELAPSED Time in its calculation
fn get_elapsed_time() -> Duration {

    let t1 = Instant::now(); 
    for i in 0..1000{
        std::thread::sleep(Duration::from_millis(10)); 
    }

    let t2 = t1.elapsed();

    t2
}