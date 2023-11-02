use rayon::prelude::*;

fn main() {

    // Mutate items in parallel
    let mut arr = [01, 2, 3, 4, 5, 6];
    arr.par_iter_mut().for_each(|p| *p += 1);
    println!("{:?}", arr);

    // Test if any or all elements of a collection matchs a given predicate. 

    let mut ages = vec![3,45,40,20,24,12, 8,9]; 

    println!("All Even Ages -> {:?} ", ages.par_iter().all(|p| p > &0)); // All matches the condition. 
    println!("Any Even Ages -> {:?} ", ages.par_iter().any(|p| p%2 == 0)); // Any matches the condition. 
}
