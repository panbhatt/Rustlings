// Mutex , its kind of same as RefCell.
use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(100);
    println!("{:?}", my_mutex); // This will show it's action value.
    let mut my_mutex_changer = my_mutex.lock().unwrap(); // Since this is RESULT.
    println!("{:?}", my_mutex); // This will show it is LOCKED>
    println!("{:?}", my_mutex_changer);

    // Change the MUTEX value
    *my_mutex_changer = 500;
    std::mem::drop(my_mutex_changer);
    println!("{:?}", my_mutex);

    println!("======================= ANOTHER WAY ================");
    // This code will change the value in the similar way a above.
    {
        let mut my_mutex_changer_1 = my_mutex.lock().unwrap();
        *my_mutex_changer_1 = 1000;
    }
    println!("{:?}", my_mutex);

    println!("======================= SIMPLEST WAY - IN ONE LINE ================");
    *my_mutex.lock().unwrap() = 50000;
    *my_mutex.lock().unwrap() = 5000000;
    println!("{:?}", my_mutex);
}
