use std::num::ParseIntError; 

fn give_number(input : &str) -> Result<i64, ParseIntError> {
    input.parse::<i64>()
}

/* This main is correct. 
fn main()  {  // for ? to work in a function, you have to make sure the function is returning -> Result<i64, ParseIntError> that satisfies the TERMINATION. 
    println!("{:?}", give_number("8989"));
    println!("{:?}", give_number("-1000"));
    
}*/

fn main() -> Result<(), ParseIntError> {
    println!("{:?}", give_number("-1000"));
    println!("{:?}", give_number("ELEVENT")?);   // An Error will be raised here. that will early return. 
    
    Ok(())
}