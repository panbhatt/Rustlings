// Float functions.
use std::cmp;  

// .floor() -> next lowest
// .ceil() -> next highest
// .round() -> if over .5 go to next number or to the lowest next number. 
// .trunc() -> Return the integer part. 

fn four_operations(input :f64) {
    println!("  Floor = {}  Ceil = {}, Round = {}, Trunc = {}", input.floor(), input.ceil(), input.round(), input.trunc());


}

fn main() {
    
    four_operations(100.34); 
    four_operations(34.89); 

    // MIN / MAX
    // std::cmp::min

    println!("Float Min Number {} , \nFloat Max number {}", f64::MIN, f64::MAX); 

    println!(" Min of 10 and 1000 is - {}", cmp::min(10,100)); 
    println!(" Max of 10 and 1000 is - {}", cmp::max(10,100)); 

    // FINDING THE MIN/MAX number of elements from a Vector. 
    let vec_of_numbers = vec![1223.33, 234.33, 20.0, 40.0, 500.0, 6000.0, -5555.4, 333.2]; 

    let minimum = vec_of_numbers.iter().fold(f64::MIN, |current_no, next_num| current_no.max(*next_num)); 
    let maximum = vec_of_numbers.iter().fold(f64::MAX, |current_no, next_num| current_no.min(*next_num)); 

    println!("MINIMUM = {}, maximum = {}", minimum, maximum);
    


}