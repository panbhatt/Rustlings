// IMPL Trait  to return closures from functions. 

use std::fmt::Display; 

// Both of these functions are same, and do same thing. 

fn print_item<T: Display + Into<String>>(input : T) {
    println!("{}", input); 
}

fn print_item_impl(input : impl Display + Into<String>) {
    println!("Trait -> {}", input); 
}

fn return_a_closure(input : &str) -> impl FnMut(i32) -> i32 {
     match input {
        "double" => |mut number| {
            number *= 2; 
            println!("Double number is {}", number); 
            number
        }, 
        "triple" => |mut number| {
            number *= 3; 
            println!("Triple number is {}", number); 
            number
        }, 
        _ => |mut number| {
            println!("Sorry, I do not understand anything else then DOUBLE/TRIPLE"); 
            number
        }
     }
}

fn main() {

    let name = "Pankaj Bhatt - str"; 
    let name_string = String::from("Pankaj Bhatt - STRING "); 

    print_item(name); 
    print_item(name_string.clone()); 

    print_item_impl(name); 
    print_item_impl(name_string); 

    println!("================= RETURN CLOSURE CODE ==================="); 

    let mut double_fn = return_a_closure("double"); 
    let mut triple_fn = return_a_closure("triple"); 
    println!("{}", double_fn(10)); 
    println!("{}", triple_fn(10)); 

}