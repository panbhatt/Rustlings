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
    println!("{:?}", take_fifth(short_ages.clone())); // Here we are going to get NONE.

    // Unwrap to get the value.
    println!("{:?}", take_fifth(ages.clone()).unwrap()); // If unwrap returns NONE, it would panic.

    //println!("{:?}", take_fifth(short_ages.clone()).expect("Absolutely need at least 5 items to work")); // Use of Expect function. Panic with this message.

    // Better way is to use MATCH.
    let fifth_ele = take_fifth(short_ages.clone());
    match fifth_ele {
        Some(number) => println!("{:?}", number),
        None => println!("Error -> Absolutely need at least 5 items to work from MATCH "),
    }

    handle_option(fifth_ele);
    handle_option(take_fifth(ages.clone()));

    // Us eof is_some and is_none function. 
    let mut ele = take_fifth(ages.clone()); 
    if ele.is_some() {
        println!("VALUE {:?}", ele.unwrap());
    } else {
        println!(" -> It is NONE");
    }

    println!("====================================="); 

    let ele = take_fifth(short_ages.clone()); 
    println!("{:?}", ele.unwrap_or(-1 ));
    println!("{:?}", ele.unwrap_or_default()); // will give default fo the TYpe of T present in the option 

}

// Generalization of the Option handler.
fn handle_option(option: Option<i32>) {
    match option {
        Some(number) => println!("{:?}", number),
        None => println!("NONE"),
    }
}
