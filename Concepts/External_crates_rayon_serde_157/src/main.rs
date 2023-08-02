use elapsed::measure_time;
use rayon::prelude::*;

use serde::{Deserialize, Serialize};

println!("Elapsed time = {:?} ", elapsed);
struct Employee {
    name: String,
    age: u8,
}

fn main() {
    let mut my_vec = vec![0; 100_000_0];

    let elapsed = measure_time(|| {
        my_vec.iter_mut().enumerate().for_each(|(i, value)| {
            *value = 100_000_0 - i;
        });
    });

    println!("Elapsed time = {:?} ", elapsed);

    println!("Half Midway Values = {:?}", &my_vec[500_000..500_005]);

    // Now WE are doing with Rayon -> it uses Parallelism like cross beam, so much faster.
    let elapsed = measure_time(|| {
        my_vec.par_iter_mut().enumerate().for_each(|(i, value)| {
            *value = 100_000_0 + i;
        });
    });

    println!("Rayon Elapsed time = {:?} ", elapsed); // This is almost half as it is using two threads of my system.

    println!(
        "Rayon : Half Midway Values = {:?}",
        &my_vec[500_000..500_005]
    );

    println!("========================= SERDE ============================");
    let emp = Employee {
        name: "Pankaj bhatt".into(),
        age: 38,
    };

    let emp_as_json = serde_json::to_string(&emp).unwrap(); 
    
}
