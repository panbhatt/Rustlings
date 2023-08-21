fn main() {
    let parse_number = "90".parse::<i32>();

    println!("Result is -> {:?}", parse_number);
    if let Ok(num) = parse_number {
        println!("\tNumber -> {:?}", num); 
    }

    // Inverse of above IF LET is LET ELSE. 

    println!("========================================="); 

    let parsed_number = "90x".parse::<i32>();
    let Ok(invalid_num) = parsed_number else {
        println!("Invalid number entered to parse: {:?}", parsed_number); 
        return ; 

    }; 

}
