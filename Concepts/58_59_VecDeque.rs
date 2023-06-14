use std::collections::VecDeque;

fn main() {
    /*let mut my_vec = vec![122 ; 600_000];

    for i in 0..600000 {
        my_vec.remove(0);
    }

    println!("LENGTH = {}", my_vec.len());*/

    // The above code , will take a lot of time to run, as in every loop , it is going to shift n-1 elements to the left. around 18 seconds.

    // ALTERNATIVE is VECDEQUE -> u can take from the front or from the right.

    let mut my_vec = VecDeque::from(vec![12; 600_000]);
    for _i in 0..600000 {
        my_vec.pop_front();
    }

    // THe Above code finishes in 0.428 seconds.

    let mut my_vec_deque = VecDeque::new();
    let my_ages = vec![10,20,30,40,50   ]; 
    for age in my_ages {
        my_vec_deque.push_front(age); // to put at the front. 
    }
}
