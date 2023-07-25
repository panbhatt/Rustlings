// Channels with Threads.

use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (sender, receiver) = channel(); // Arc Mutex can be used here.
    let sender2 = sender.clone();
    let sender3 = sender.clone();

    let mut handle_vec = vec![];

    handle_vec.push(spawn(move || {
        sender.send(10);
    }));

    handle_vec.push(spawn(move || {
        sender2.send(100);
    }));

    handle_vec.push(spawn(move || {
        sender3.send(1000);
    }));

    // Without this, our main thread is not going to be joined by child thread.
    for handle in handle_vec {
        handle.join();
    }

    // Since the threads can be executed in any order, the output is unpredictable

    println!("{:?}", receiver.try_recv());
    println!("{:?}", receiver.try_recv());
    println!("{:?}", receiver.try_recv());
}
