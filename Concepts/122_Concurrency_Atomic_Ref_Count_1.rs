// ARC -> Implementation of Atomic Reference Count.

// This is used, as it is Thread Safe to send the ARC across multiple threads. IT implements the send and SYNC traits,

use std::thread::spawn;
use std::rc::Rc; 
use std::cell::RefCell; 

// THis CODE WLL GENERATE AN ERROR and that is expected. 

fn main() {
    let my_number = Rc::new(RefCell::new(10));
    let mut handlers_vec = vec![];

    for i in 0..10 {
        let  my_n1 = Rc::clone(&my_number);
        let handle = spawn(move || {
            *my_n1.borrow_mut() += 1; // Here they access same data. 
        });
        handlers_vec.push(handle);
    }

    for handle in handlers_vec {
        handle.join().unwrap();
    }

    // The Above code generates this error: Rc<RefCell<i32>>` cannot be sent between threads safely and wont' work. 
}
