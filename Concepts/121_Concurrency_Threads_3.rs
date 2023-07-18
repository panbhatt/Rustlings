use std::thread::spawn;

fn main() {
    println!("=============== PANICKING within the Thread========================");
    // THis is serialized way of handling threads which is not good.
    for i in 0..10 {
        let handle = spawn(move || {
            if i % 2 == 0 {
                panic!("I dont like EVEN NUMBERS")
            } else {
                i // this will return the number itself.
            }
        });

        println!("Thread No -> {} Result -> {:?} ", i, handle.join()); // Join() method returns an Resut<Ok,Err<T>>.
    }

    // Above is not true concurrency as we will always get the same result.
    println!("=============== REAL CONCURRENCY - Threads in no order ========================");
    let mut handlers_vec = vec![];
    for i in 0..10 {
        let handle = spawn(move || {
            println!("Handling thread {:?}", i);
            i
        });
        handlers_vec.push(handle);
    }

    for handle in handlers_vec {
        println!("{:?}", handle.join().unwrap())
    }
}
