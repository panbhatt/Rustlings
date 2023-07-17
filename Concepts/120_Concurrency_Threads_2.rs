use std::thread::spawn;

// There are three kinds of Closures.
//Fn -> takes a reference
//FnMut -> takes a mutable reference
// FnOnce -> takes ownership (as shown below)

fn main() {
    //Joining on the Main thread, so that main thread will wait.

    let thread1 = spawn(|| println!("DOING HEAVY WEiGHT TASK"));

    let thread2 = spawn(|| println!("DOING LIGHT WEiGHT TASK"));

    thread1.join();
    thread2.join();

    println!("==============MOVE - take ownership=========================");
    for i in 0..10 {
        let th1 = spawn(move || {
            println!("From Thread 1 -> {}", i);
        });
        let th2 = spawn(move || {
            println!("From Thread  2 -> {}", i);
        });
        
        th2.join();
        th1.join();
    }
}
