use std::thread::spawn;

fn main() {
    spawn(|| {
        println!("HELLO 1st Thread");
    });

    spawn(|| {
        println!("HELLO 2st Thread");
    });

    // so that our main thread will wait.
    for _ in 0..=10000000 {
        let _x = 9;
    }
}
