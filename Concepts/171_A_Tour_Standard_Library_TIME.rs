// Standard Time.

// Use external CHRONO if you want a full fledged time library.

use std::time::{Duration, Instant};

fn main() {
    let time1 = Instant::now();

    println!("{:?}", time1); // Print Instant { tv_sec: 20022, tv_nsec: 565173496 }

    let time2 = Instant::now();

    println!("{:?}", time2 - time1);
}
