// Channel -< it is same as tha of golang.

use std::sync::mpsc::channel;

#[warn(unused_must_use)]

fn main() {
    let (sender, receiver) = channel();

    // we can send multiple and even ignore the send message alltogether.
    let send_result = sender.send("Pankaj Bhatt"); // You can use UNWRAP method to get the value or ignore it.
    match send_result {
        Ok(s) => println!(" Succesfully send. -> {:?}", s),
        Err(s) => println!(" Error Occured -> {:?} ", s),
    }

    println!("{:?}", receiver.try_recv());
    // if we dint send anything, let see what output it gave, again it will give result.

    let empty_recv_result = receiver.try_recv();
    match empty_recv_result {
        Ok(s) => println!("Receive Empty value, {}", s),
        Err(s) => println!("Received Empty value, -> {}", s),
    }
}
