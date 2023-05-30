// Mutable references with &mut

fn main() {
    let mut number :isize = 10203043; 
    //let num_ref = &mut number; // Tis wont work, this needs to pushed down, plase see below, it has been copied, otherwise it is an Error. 
    // You can't have one mutable and any other mutable reference. Although u can have multiple Immutable references. 
    println!("Before Number -> {}", number); 
    let num_ref = &mut number;
    *num_ref += 100; 
    println!("After Number -> {}", number); 

    
    // Even if we are reassigning a reference of a variable and reassigning it, still we can use it via reference. 
    let country = String::from("US"); 
    let country_ref = &country; 
    let country = 10; 

    println!("After Country -> {} Size = {} ", country_ref ,country);




}