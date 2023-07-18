// ARC -> Implementation of Atomic Reference Count.

// This is used, as it is Thread Safe to send the ARC across multiple threads. IT implements the send and SYNC traits,

// This is a continuation of the existing 122 File where we replace RC with ARC.
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

fn main() {
    let my_number = Arc::new(Mutex::new(10)); // RwLock can e used here. 
    let mut handlers_vec = vec![];

    for _i in 0..10 {
        let my_n1 = Arc::clone(&my_number);
        let handle = spawn(move || {
            *my_n1.lock().unwrap() += 10; // replace lock by write for RwLock
        });
        // Number of owners at a specific point of time. 

        handlers_vec.push(handle);

        println!("Number of owners: {}", Arc::strong_count(&my_number)); // This will give number of owners. 
    }

    for handle in handlers_vec {
        handle.join();
    }

    println!("Final Sum = {:?}", my_number); // This will print 110 at the end of the program, which proves it is doing the sharing of data amongst threads correctly. 
}
