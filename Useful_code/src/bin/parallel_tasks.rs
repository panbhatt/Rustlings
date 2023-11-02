use rand::{thread_rng, distributions::Alphanumeric, Rng};
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

    // Using any ().  Find first EVEN Number. Similary , we have find_first and first_last()  too.
    println!("Divide by 3 ->  {:?}", ages.par_iter().find_any(|p| *p % 3 == 0));
    println!("Divide by 15 ->  {:?}", ages.par_iter().find_any(|p| *p % 15 == 0));

    // UNSORTEd Vector in parallel. 
    let mut unsorted_vec = vec![String::new(); 50]; 
    unsorted_vec.par_iter_mut().for_each(|p|  {
        let  rng = thread_rng(); 
        *p = rng.sample_iter(&Alphanumeric).take(7).map(char::from).collect(); 
    });

    unsorted_vec.sort_unstable();
    println!("SORTED STRING -> {:?}", unsorted_vec); 

}
