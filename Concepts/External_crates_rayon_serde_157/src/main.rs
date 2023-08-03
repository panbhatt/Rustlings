use elapsed::measure_time;
use rayon::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json; 


#[derive(Debug, Serialize, Deserialize)]
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
    println!("JSON = {}", emp_as_json);

    // Trying to get the Struct back from JSON. 
    let mut new_emp : Employee = serde_json::from_str(&emp_as_json).unwrap(); 
    new_emp.name = "Rahul Gupta".into(); 
    new_emp.age = 45; 
    println!("Parse New Employee = {:?}", new_emp); 
    
}
