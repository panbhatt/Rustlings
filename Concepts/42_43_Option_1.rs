// Option is an ENUM

/* This is the definition of OPTION.
pub enum Option<T> {
    None,
    Some(T)
}

*/

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    // Rather then returning i32 we are returning an OPTION.
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let ages = [23, 34, 32, 37, 38, 40].to_vec();
    println!("{:?}", take_fifth(ages.clone())); // Here we are going to get teh value 38

    let short_ages = ages[0..2].to_vec();
    println!("{:?}", take_fifth(short_ages)); // Here we are going to get NONE.

    // Unwrap to get the value.
    println!("{:?}", take_fifth(ages.clone()).unwrap()); // If unwrap returns NONE, it would panic.
}
