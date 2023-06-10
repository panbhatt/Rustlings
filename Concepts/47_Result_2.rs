// TUROBFISH & FIND THE ERROR TYPE

use std::num::ParseIntError;

fn parse_integer(s: &str) -> Result<isize, ParseIntError> {
    s.parse::<isize>() // THis is called TURBOFISH ::<> because it looks like turbofish.
}

fn main() {
    let my_number = parse_integer("123456789");
    if my_number.is_ok() {
        println!("{}", my_number.unwrap());
    } else {
        println!("No Valid Number passed");
    }

    println!("=========================================");
    let numbers = vec!["5", "Pankaj", "67", "sixtyseven", "eight", "8989"];

    for number in numbers {
        let result = parse_integer(number);
        let result_number = match result {
            Ok(n) => println!("{}", n),
            Err(e) => println!("{}", e),
        };
    }
}
