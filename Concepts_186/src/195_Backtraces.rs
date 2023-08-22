// How to use and set backtraces.

use std::backtrace::{Backtrace, BacktraceStatus};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    /* PART 1
    env::set_var("RUST_BACKTRACE", "1"); // FOR SMALL BACKTRACE
    env::set_var("RUST_BACKTRACE", "full"); // FOR BIG BACKTRACE
    panic!(" PANIC OCCURED ");*/

    // Part 2.  Captaure backtrace on our own.
    /*env::set_var("RUST_BACKTRACE", "1");
    println!(
        "BACKTRACE :: -> \n {}",
        std::backtrace::Backtrace::capture() // or it can be force_capture
    );*/

    // part 2.
    std::panic::set_hook(Box::new(|x| {
        println!("Panicked!! Trying to capture a backtrace ... ");
        let btc = Backtrace::capture();
        match btc.status() {
            BacktraceStatus::Captured => println!("Got BackTrace => {btc:#?}"),
            BacktraceStatus::Disabled => println!(" Backtrace Disabled"), // WIL OCCUR if RUST_BACKTRACE is not enabled.
            BacktraceStatus::Unsupported => println!("Backtrace Unsupported"),
            _ => println!("Enum not supported"),
        };
    }));

    panic!("PANIC OCCURED");
}
