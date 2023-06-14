// TRY! Macro , it should be used via a QUESTION mark operator. 

use std::num::ParseIntError;

fn parse_number(str: &str) -> Result<i32, ParseIntError> {

    let parsed_number = str.parse::<i32>()?;  // This is the way we have to do it. FISH OPERATOR with ?
    Ok(parsed_number)
}


fn main() {
    let my_vec = vec!["SEven", "56", "Elevelnt", "23"]; 

    for item in &my_vec {
        println!("{:?}", parse_number(item));
    }
    println!("==============================MULTIPLE CONCATENATED DATA==========================="); 
    for item in &my_vec {
        println!("{:?}", parse_number1(item));
    }
}

// THis is another variant where we can use DOT operator to extend its capabilities
fn parse_number1(str: &str) -> Result<u8, ParseIntError> {

    let parsed_number = str.parse::<i32>()?.to_string().parse::<u32>()?.to_string().parse::<u8>()?;  // FISH OPERATOR with ?
    Ok(parsed_number)
}