// Result is cousin of ENUM and is similar but used mostly, since it can contain Errors.

use std::result::Result;

// Option may there is something may be nothing.
// Result -> may it wil work or will give an Error.

// () -> Can be used to represent NOTHING.
/*
enum Option<T>{
    Some(T),
    None
}

is_some()
is_None()

enum Result<T,E>{
    Ok(T),
    Err(E),
}

is_ok()
is_err()
*/

fn divide(input: i32, divisor: i32) -> Result<i32, String> {
    // The STring can be error, we wlil see errror afterwards.
    if input / divisor > 0 {
        Ok(input / divisor)
    } else {
        Err("An Error occured, while dividing".to_string())
    }
}

fn main() {
    let result = divide(10, 2); // This will return the dividend
    if result.is_ok() {
        println!("RESULT IS = {}", result.unwrap());
    } else if result.is_err() {
        println!("Error is {:?}", result.err());
    }

    // How to handle the Error and result via using MATCH.
    let result1 = divide(0, 10); // Since it is 0/10 , we are going to get an Error.
    match result1 {
        Ok(number) => println!(" Dividend is { }", number),
        Err(err) => println!(" Error is {:?} ", err),
    }
}
