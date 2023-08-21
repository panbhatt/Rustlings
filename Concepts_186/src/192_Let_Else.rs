fn main() {
    let parse_number = "90".parse::<i32>();

    println!("Result is -> {:?}", parse_number);
    if let Ok(num) = parse_number {
        println!("\tNumber -> {:?}", num); 
    } else {
        println!("This would never get printed"); 
    }

    // Inverse of above IF LET is LET ELSE. This is just an improvement over the above ELSE pattern. 

    println!("========================================="); 

    let parsed_number = "90x".parse::<i32>();
    let Ok(invalid_num) = parsed_number else {
        println!("Invalid number entered to parse: {:?}", parsed_number); 
        return ; 

    }; 

}
